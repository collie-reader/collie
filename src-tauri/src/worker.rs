use std::thread;
use std::time;
use tauri::AppHandle;
use tauri::Manager;

use tauri::api::notification::Notification;

use crate::models::settings;
use crate::models::settings::SettingKey;
use crate::{models::items::ItemToCreate, producer::create_new_items};

pub fn start(app_id: String, handle: AppHandle) {
    let polling_frequency = settings::read(SettingKey::PollingFrequency)
        .map(|x| x.value)
        .unwrap_or("120".to_string())
        .parse()
        .unwrap_or(120);
    let polling_frequency = if polling_frequency < 30 {
        30
    } else {
        polling_frequency
    };

    thread::spawn(move || loop {
        let inserted = create_new_items();
        if !inserted.is_empty() {
            notify(&app_id, &inserted);
            let _ = handle.emit_all("feed_updated", ());
        }

        thread::sleep(time::Duration::from_secs(polling_frequency));
    });
}

fn notify(app_id: &str, args: &[ItemToCreate]) {
    if args.len() <= 3 {
        for arg in args {
            let _ = Notification::new(app_id)
                .title(&arg.title)
                .body(&arg.description)
                .show();
        }
    } else {
        let _ = Notification::new(app_id)
            .title("New items arrived")
            .body(format!("There are {} items to read", args.len()))
            .show();
    }
}
