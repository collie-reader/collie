use chrono::DateTime;
use collie::model::item::{ItemStatus, ItemToCreate};
use collie::repository::database::DbConnection;
use collie::worker::Worker;
use regex::Regex;
use tauri::api::notification::Notification;
use tauri::App;
use tauri::Manager;

use crate::fetchers;
use crate::fetchers::auth::AuthClient;
use crate::models::settings;
use crate::models::settings::{SettingKey, SettingToUpdate};

#[tokio::main]
pub async fn start(conn: DbConnection, app: &App) {
    let app_handle = app.handle();
    let app_id = app.config().tauri.bundle.identifier.clone();
    let upstream_url = settings::upstream_url(&conn);
    let upstream_credentials = settings::upstream_credentials(&conn);

    let worker = if upstream_url.is_some() && upstream_credentials.is_some() {
        None
    } else {
        Some(Worker::new(conn.clone(), proxy(&conn)))
    };

    let auth_client =
        if let (Some(url), Some((access, secret))) = (&upstream_url, upstream_credentials) {
            Some(AuthClient::new(url.clone(), access, secret))
        } else {
            None
        };

    tauri::async_runtime::spawn(async move {
        loop {
            let inserted = if let Some(worker) = &worker {
                worker.execute().await
            } else if let Some(client) = &auth_client {
                worker_using_upstream(&conn, client).await
            } else {
                Ok(vec![])
            };

            match inserted {
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

async fn worker_using_upstream(
    conn: &DbConnection,
    client: &AuthClient,
) -> Result<Vec<ItemToCreate>, collie::error::Error> {
    let last_sync_time = settings::read(conn, &SettingKey::UpstreamLastSyncTime)
        .ok()
        .and_then(|s| DateTime::parse_from_rfc3339(&s.value).ok());

    let opt = collie::model::item::ItemReadOption {
        ids: None,
        feed: None,
        status: Some(ItemStatus::Unread),
        is_saved: None,
        order_by: None,
        limit: None,
        offset: None,
    };

    match fetchers::items::read_all(client, &opt).await {
        Ok(items) => {
            let new_items: Vec<ItemToCreate> = items
                .iter()
                .filter(|x| last_sync_time.map(|t| x.published_at > t).unwrap_or(true))
                .map(|x| ItemToCreate {
                    author: x.author.clone(),
                    title: x.title.clone(),
                    description: x.description.clone(),
                    link: x.link.clone(),
                    status: x.status.clone(),
                    published_at: x.published_at,
                    feed: x.feed.id,
                })
                .collect();

            if let Some(latest) = items.iter().map(|x| x.published_at).max() {
                let _ = settings::update(
                    conn,
                    &SettingToUpdate {
                        key: SettingKey::UpstreamLastSyncTime,
                        value: latest.to_rfc3339(),
                    },
                );
            }

            Ok(new_items)
        }
        Err(_) => Err(collie::error::Error::BadArgument),
    }
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
        let html_tag_regex = Regex::new(r"<.*?>").unwrap();
        for arg in args {
            let _ = Notification::new(app_id)
                .title(&arg.title)
                .body(html_tag_regex.replace_all(&arg.description, ""))
                .show();
        }
    } else {
        let _ = Notification::new(app_id)
            .title("New items arrived")
            .body(format!("There are {} items to read", args.len()))
            .show();
    }
}
