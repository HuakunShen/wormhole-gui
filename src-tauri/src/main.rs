#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use magic_wormhole::{transfer, transit};
use tauri::Manager;
use wormhole_gui::{install_ctrlc_handler, produce_filename, send as send2};

#[derive(Default)]
struct WormholeManager {}

impl WormholeManager {}

#[derive(Clone, serde::Serialize)]
struct ReceiveCodePayload {
    code: String,
}

#[derive(Clone, serde::Serialize)]
struct Progress {
    sent: u64,
    total: u64,
}

#[tauri::command]
async fn send(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, WormholeManager>,
    filepath: String,
) -> Result<(), String> {
    let path = PathBuf::from(filepath);

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
    println!("{:?}", relay_hints);
    let num_words = 2;
    let app_config = transfer::APP_CONFIG;
    let (server_welcome, connector) =
        magic_wormhole::Wormhole::connect_without_code(app_config, num_words)
            .await
            .unwrap();

    println!("{:?}", server_welcome);
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
    let ctrl_c = install_ctrlc_handler().unwrap();
    let progress_handler = move |sent, total| {
        main_window
            .emit("wormhole://progress", Progress { sent, total })
            .unwrap();
        // println!("{} / {}", sent, total);
    };
    send2(
        wormhole,
        relay_hints,
        &path,
        filename.as_ref(),
        transit_abilities,
        ctrl_c.clone(),
        progress_handler,
    )
    .await
    .unwrap();

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send])
        .setup(|app| {
            {
                let window = app.get_window("main").unwrap();
                app.manage(WormholeManager::default());
                window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
