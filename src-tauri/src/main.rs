// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models {
    pub mod database;
    pub mod feeds;
}

pub mod commands {
    pub mod feeds;
}

fn main() {
    let _ = models::database::migrate();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::feeds::create_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
