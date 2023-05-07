use std::path::PathBuf;
use magic_wormhole::{transfer, transit, Wormhole};
use std::ffi::{OsStr, OsString};
use color_eyre::eyre::WrapErr;
use color_eyre::{eyre, eyre::Context, Report};


pub fn produce_filename(file_path: &PathBuf) -> OsString {
    let mut file_name = file_path.file_name().unwrap();
    if file_path.is_dir() {
        let mut x = file_name.to_os_string();
        x.push(".tar");
        x
    } else {
        file_name.to_os_string()
    }
}


pub fn install_ctrlc_handler(
) -> eyre::Result<impl Fn() -> futures::future::BoxFuture<'static, ()> + Clone> {
    use async_std::sync::{Condvar, Mutex};
    use color_eyre::eyre::WrapErr;
    use futures::FutureExt;
    use std::sync::Arc;

    let notifier = Arc::new((Mutex::new(false), Condvar::new()));

    /* Register the handler */
    let notifier2 = notifier.clone();
    ctrlc::set_handler(move || {
        futures::executor::block_on(async {
            let mut has_notified = notifier2.0.lock().await;
            if *has_notified {
                /* Second signal. Exit */
                // log::debug!("Exit.");
                std::process::exit(130);
            }
            /* First signal. */
            // log::info!("Got Ctrl-C event. Press again to exit immediately");
            *has_notified = true;
            notifier2.1.notify_all();
        })
    })
    .context("Error setting Ctrl-C handler")?;
    let ret = move || {
        /* Transform the notification into a future that waits */
        let notifier = notifier.clone();
        async move {
            let (lock, cvar) = &*notifier; // get notifier content
            let mut started = lock.lock().await;
            while !*started { // while not notified, keep waiting
                started = cvar.wait(started).await;
            }
            // exist the loop, means notified, started = true
        }
        .boxed()
    };
    Ok(ret)
}


pub async fn send(
    wormhole: Wormhole,
    relay_hints: Vec<transit::RelayHint>,
    file_path: &PathBuf,
    file_name: &OsStr,
    transit_abilities: transit::Abilities,
    cancel_callback: impl futures::Future<Output = ()>,
    progress_handler: impl Fn(u64, u64) + 'static,
) -> eyre::Result<()> {
    // let pb = create_progress_bar(0);
    // let pb2 = pb.clone();
    transfer::send_file_or_folder(
        wormhole,
        relay_hints,
        file_path,
        file_name,
        transit_abilities,
        &transit::log_transit_connection,
        progress_handler,
        // move |sent, total| {
        //     // if sent == 0 {
        //     //     pb.reset_elapsed();
        //     //     pb.set_length(total);
        //     //     pb.enable_steady_tick(std::time::Duration::from_millis(250));
        //     // }
        //     // pb.set_position(sent);
        // },
        cancel_callback,
        // futures::future::pending()
    )
    .await
    .context("Send process failed")?;
    // pb2.finish();
    Ok(())
}


// pub async fn receive()


// pub async fn wormhole_send_file(filepath: &PathBuf) {
//     // let filename = filepath.file_name().unwrap();
//     let filename = produce_filename(filepath);
//     let mut relay_hints: Vec<transit::RelayHint> = Vec::new();
//     relay_hints.push(
//         transit::RelayHint::from_urls(
//             None,
//             [magic_wormhole::transit::DEFAULT_RELAY_SERVER
//                 .parse()
//                 .unwrap()],
//         )
//         .unwrap(),
//     );
//     println!("{:?}", relay_hints);
//     let num_words = 2;
//     let app_config = transfer::APP_CONFIG;
//     let (server_welcome, connector) =
//         magic_wormhole::Wormhole::connect_without_code(app_config, num_words)
//             .await
//             .unwrap();

//     println!("{:?}", server_welcome);
//     // let mut clipboard = ClipboardContext::new()
//     //     .map_err(|err| {
//     //         log::warn!("Failed to initialize clipboard support: {}", err);
//     //     })
//     //     .ok();
//     // if let Some(mut clipboard) = clipboard {
//     //     match clipboard.set_contents(server_welcome.code.to_string()) {
//     //         Ok(()) => log::info!("Code copied to clipboard"),
//     //         Err(err) => log::warn!("Failed to copy code to clipboard: {}", err),
//     //     }
//     // }

//     println!("Receive Code Written to Clipboard");
//     let wormhole = connector.await.unwrap(); // will block until the receiver has connected
//     println!("{:?}", wormhole);
//     let code = server_welcome.code;
//     println!("{:?}", code);

//     // let filename = std::ffi::OsStr::new("README.md");
//     let transit_abilities = transit::Abilities::ALL_ABILITIES;
//     let ctrl_c = install_ctrlc_handler().unwrap();
//     send(
//         wormhole,
//         relay_hints,
//         &filepath,
//         filename.as_ref(),
//         transit_abilities,
//         ctrl_c.clone(),
//     )
//     .await
//     .unwrap();
// }
