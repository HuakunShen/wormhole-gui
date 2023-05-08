#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::PathBuf, str::FromStr};
use async_std::fs::OpenOptions;
use async_std::sync::{Condvar, Mutex};
use color_eyre::eyre::Context;
use magic_wormhole::{transfer, transit, Wormhole};
use std::error::Error;
use std::sync::Arc;
use tauri::Manager;
use wormhole_gui::{produce_filename, send as send2};

// struct WormholeManager {
//     app_handle: tauri::AppHandle,
//     cancel_mutex: Arc<Mutex<bool>>,
// }
struct Counter(Mutex<i32>);

struct WormholeManager {
    pub app_handle: tauri::AppHandle,
    pub cancel_mutex: Arc<(Mutex<bool>, Condvar)>,
}
impl WormholeManager {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            app_handle: app_handle,
            cancel_mutex: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct ReceiveCodePayload {
    code: String,
}

#[derive(Clone, serde::Serialize)]
struct Progress {
    sent: u64,
    total: u64,
}

// #[tauri::command]
// async fn cancel(
//     app_handle: tauri::AppHandle,
//     manager: tauri::State<'_, WormholeManager>,
// ) -> Result<(), String> {
//     let mut cancelled = manager.0.lock().unwrap();
//     *cancelled = true;
//     // Mutex is automatically unlocked when you exit the block
//     // Lock the counter(Mutex) to get the current value
//     // let mut ct = counter.0.lock().unwrap();
//     // Change and return a new value
//     // *ct += count_val;
//     // *ct
//     // Mutex is automatically unlocked when you exit the block
//     Ok(())
// }

#[tauri::command]
async fn send(
    app_handle: tauri::AppHandle,
    filepath: String,
    state: tauri::State<'_, WormholeManager>,
) -> Result<(), String> {
    let path = PathBuf::from(filepath);
    // let cancelled = state.cancel_mutex.lock().unwrap();
    // state.cancel_mutex.0.lock().await;
    let notifier = state.cancel_mutex.clone();
    // let manager_mutex = state.lock().unwrap();
    // let cancelled = manager_mutex.cancel_mutex.lock().unwrap();
    // let cancel_lock = state.cancel_mutex.lock().unwrap();
    // if *cancel_lock {
    //     return Err("Another Task In Progress".to_string());
    // }
    let filename = produce_filename(&path);
    let mut relay_hints: Vec<transit::RelayHint> = Vec::new();
    relay_hints.push(
        transit::RelayHint::from_urls(
            None,
            [magic_wormhole::transit::DEFAULT_RELAY_SERVER
                .parse()
                .unwrap()],
        )
        .unwrap(),
    );
    let num_words = 2;
    let app_config = transfer::APP_CONFIG;
    let (server_welcome, connector) =
        magic_wormhole::Wormhole::connect_without_code(app_config, num_words)
            .await
            .map_err(|e| e.to_string())
            .unwrap();

    let code = server_welcome.code;

    let main_window = app_handle.get_window("main").unwrap();
    main_window
        .emit(
            "wormhole://receive-code",
            ReceiveCodePayload {
                code: code.clone().0,
            },
        )
        .unwrap();
    let wormhole = connector.await.unwrap(); // will block until the receiver has connected

    // let filename = std::ffi::OsStr::new("README.md");
    let transit_abilities = transit::Abilities::ALL_ABILITIES;
    // impl Fn() -> Pin<Box<dyn Future<Output = ...> + Send>> + CLone
    // let ctrl_c = install_ctrlc_handler().unwrap();
    let t = std::cell::Cell::new(std::time::Instant::now());
    let interval = std::time::Duration::from_millis(300);

    let progress_handler = move |sent, total| {
        if t.get().elapsed() < interval && sent != total {
            return;
        }
        t.set(std::time::Instant::now());
        main_window
            .emit("wormhole://progress", Progress { sent, total })
            .unwrap();
    };
    send2(
        wormhole,
        relay_hints,
        &path,
        filename.as_ref(),
        transit_abilities,
        // ctrl_c.clone(),
        futures::future::pending(),
        progress_handler,
    )
    .await
    .unwrap();

    Ok(())
}

#[tauri::command]
async fn receive(
    app_handle: tauri::AppHandle,
    receive_code: String,
    save_dir: String,
    state: tauri::State<'_, WormholeManager>,
) -> Result<String, String> {
    let transit_abilities = transit::Abilities::ALL_ABILITIES;
    let mut relay_hints: Vec<transit::RelayHint> = Vec::new();
    relay_hints.push(
        transit::RelayHint::from_urls(
            None,
            [magic_wormhole::transit::DEFAULT_RELAY_SERVER
                .parse()
                .unwrap()],
        )
        .unwrap(),
    );
    let app_config = transfer::APP_CONFIG;
    let code = magic_wormhole::Code(receive_code);
    let (server_welcome, wormhole) = Wormhole::connect_with_code(app_config, code).await.unwrap();
    println!("{:?}", server_welcome);
    let req = transfer::request_file(
        wormhole,
        relay_hints,
        transit_abilities,
        futures::future::pending(),
    )
    .await
    .context("Could not get an offer")
    .unwrap()
    .unwrap();

    let main_window = app_handle.get_window("main").unwrap();
    let progress_handler = move |sent, total| {
        main_window
            .emit("wormhole://progress", Progress { sent, total })
            .unwrap();
        // println!("{} / {}", sent, total);
    };

    let req_filename = req.filename.file_name().unwrap();
    let mut save_dir_pathbuf = PathBuf::from(&save_dir);
    save_dir_pathbuf.push(req_filename);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&save_dir_pathbuf.clone())
        .await
        .unwrap();
    req.accept(
        &transit::log_transit_connection,
        progress_handler,
        &mut file,
        futures::future::pending(),
    )
    .await
    .context("Receive process failed")
    .unwrap();
    let ret = save_dir_pathbuf.to_string_lossy().into_owned();
    Ok(ret)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send, receive])
        .setup(|app| {
            {
                let window = app.get_window("main").unwrap();
                app.manage(WormholeManager::new(app.handle()));
                // window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
