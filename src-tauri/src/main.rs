// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use std::{fs, sync::Mutex};
use tauri::Manager;

pub mod models {
    pub mod database;
    pub mod feeds;
    pub mod items;
    pub mod settings;
}

pub mod commands {
    pub mod feeds;
    pub mod items;
    pub mod settings;
}

pub mod error;
pub mod producer;
pub mod syndication;
pub mod worker;

pub struct DbState {
    db: Mutex<Connection>,
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::feeds::create_feed,
            commands::feeds::read_all_feeds,
            commands::feeds::read_feed,
            commands::feeds::update_feed,
            commands::feeds::delete_feed,
            commands::items::read_all_items,
            commands::items::count_all_items,
            commands::items::update_item,
            commands::items::update_items,
            commands::settings::read_all_settings,
            commands::settings::update_setting,
        ])
        .build(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");

    let app_data_dir = app.handle().path_resolver().app_data_dir().unwrap();
    fs::create_dir_all(&app_data_dir).unwrap();
    let db = models::database::open_connection(&app_data_dir).unwrap();
    let _ = models::database::migrate(&db);

    worker::start(&app);

    app.manage(DbState { db: Mutex::new(db) });
    app.run(move |handle, event| {
        if let tauri::RunEvent::WindowEvent {
            label: _,
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } = event
        {
            let _ = handle.hide();
            api.prevent_close();
        }
    });
}
