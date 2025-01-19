use collie::model::item::ItemToCreate;
use collie::repository::database::DbConnection;
use collie::worker::Worker;
use regex::Regex;
use tauri::api::notification::Notification;
use tauri::App;
use tauri::Manager;

use crate::models::settings;
use crate::models::settings::SettingKey;

#[tokio::main]
pub async fn start(conn: DbConnection, app: &App) {
    let worker = Worker::new(conn.clone(), proxy(&conn));
    let app_handle = app.handle();
    let app_id = app.config().tauri.bundle.identifier.clone();

    tauri::async_runtime::spawn(async move {
        loop {
            match worker.execute().await {
                Ok(inserted) => {
                    if !inserted.is_empty() {
                        if notification(&conn) {
                            notify(&app_id, &inserted);
                        }

                        let _ = app_handle.emit_all("feed_updated", ());
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching new items: {}", err);
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(polling_frequency(&conn))).await;
        }
    });
}

fn proxy(conn: &DbConnection) -> Option<String> {
    match settings::read(conn, &SettingKey::Proxy) {
        Ok(x) => Some(x.value),
        Err(_) => None,
    }
}

fn polling_frequency(conn: &DbConnection) -> u64 {
    settings::read(conn, &SettingKey::PollingFrequency)
        .map(|x| x.value)
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300)
}

fn notification(conn: &DbConnection) -> bool {
    settings::read(conn, &SettingKey::Notification)
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
