// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models {
    pub mod database;
    pub mod feeds;
    pub mod items;
}

pub mod commands {
    pub mod feeds;
    pub mod items;
}

pub mod rss;
pub mod worker;

fn main() {
    let _ = models::database::migrate();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::feeds::create_feed,
            commands::feeds::read_all_feeds,
            commands::feeds::update_feed,
            commands::feeds::delete_feed,
        ])
        .build(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");

    worker::start(app.config().tauri.bundle.identifier.clone());

    app.run(|_, _| {});
}
