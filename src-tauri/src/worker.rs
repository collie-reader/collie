use regex::Regex;
use rusqlite::Connection;
use std::thread;
use std::time;
use tauri::App;
use tauri::Manager;

use tauri::api::notification::Notification;

use crate::models::database::open_connection;
use crate::models::items::ItemToCreate;
use crate::models::settings;
use crate::models::settings::SettingKey;
use crate::producer::create_new_items;

pub fn start(app: &App) {
    let app_handle = app.handle();
    let app_id = app.config().tauri.bundle.identifier.clone();

    let app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let db = open_connection(&app_data_dir).unwrap();

    thread::spawn(move || loop {
        let inserted = create_new_items(&db);
        if !inserted.is_empty() {
            if notification(&db) {
                notify(&app_id, &inserted);
            }

            let _ = app_handle.emit_all("feed_updated", ());
        }

        thread::sleep(time::Duration::from_secs(polling_frequency(&db)));
    });
}

fn polling_frequency(db: &Connection) -> u64 {
    settings::read(db, SettingKey::PollingFrequency)
        .map(|x| x.value)
        .unwrap_or("120".to_string())
        .parse()
        .unwrap_or(120)
}

fn notification(db: &Connection) -> bool {
    settings::read(db, SettingKey::Notification)
        .map(|x| x.value)
        .unwrap_or("1".to_string())
        .parse()
        .unwrap_or(true)
}

fn notify(app_id: &str, args: &[ItemToCreate]) {
    if args.len() <= 3 {
        for arg in args {
            let _ = Notification::new(app_id)
                .title(&arg.title)
                .body(
                    Regex::new(r"<.*?>")
                        .unwrap()
                        .replace_all(&arg.description, ""),
                )
                .show();
        }
    } else {
        let _ = Notification::new(app_id)
            .title("New items arrived")
            .body(format!("There are {} items to read", args.len()))
            .show();
    }
}
