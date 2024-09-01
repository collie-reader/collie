// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use models::database::insert_default_settings;
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::Manager;

pub mod models {
    pub mod database;
    pub mod settings;
}

pub mod commands {
    pub mod feeds;
    pub mod items;
    pub mod settings;
}

pub mod error;
pub mod worker;

fn main() {
    let _ = tauri::Builder::default()
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
            commands::settings::read_setting,
            commands::settings::update_setting,
        ])
        .setup(|app| {
            let app_data_dir = if cfg!(dev) {
                PathBuf::from("data")
            } else {
                app.handle().path_resolver().app_data_dir().unwrap()
            };

            fs::create_dir_all(&app_data_dir).unwrap();
            let db_file = &app_data_dir.join("collie.db");
            let db = collie::model::database::open_connection(&db_file).unwrap();
            let _ = collie::model::database::Migration::new()
                .table(collie::model::database::feeds_table())
                .table(collie::model::database::items_table())
                .table(crate::models::database::settings_table())
                .migrate(&db);
            let _ = insert_default_settings(&db);

            let conn = Arc::new(Mutex::new(db));
            app.manage(conn.clone());
            worker::start(conn.clone(), app);

            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                hide_window(&event);
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!("tauri.conf.json"));
}

#[cfg(target_os = "macos")]
fn hide_window(event: &tauri::GlobalWindowEvent) {
    let _ = event.window().app_handle().hide();
}

#[cfg(not(target_os = "macos"))]
fn hide_window(event: &tauri::GlobalWindowEvent) {
    event.window().hide().unwrap();
}
