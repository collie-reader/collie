use std::thread;
use std::time;
use tauri::AppHandle;
use tauri::Manager;

use tauri::api::notification::Notification;

use crate::{models::items::ItemToCreate, producer::create_new_items};

pub fn start(app_id: String, handle: AppHandle) {
    thread::spawn(move || loop {
        let inserted = create_new_items();
        if !inserted.is_empty() {
            notify(&app_id, &inserted);
            let _ = handle.emit_all("feed_updated", ());
        }

        thread::sleep(time::Duration::from_secs(120));
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
