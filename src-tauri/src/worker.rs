use std::thread;
use std::time;
use tauri::AppHandle;
use tauri::Manager;

use chrono::{Duration, Utc};
use tauri::api::notification::Notification;

use crate::models::feeds::FeedStatus;
use crate::syndication::RawItem;
use crate::{
    models::{
        feeds::{self, FeedToUpdate},
        items::{self, ItemStatus, ItemToCreate},
    },
    syndication::fecth_feed_items,
};

pub fn start(app_id: String, handle: AppHandle) {
    thread::spawn(move || loop {
        let pairs = get_links_to_check();

        let mut inserted = vec![];
        for (feed, link) in pairs {
            if let Ok(items) = fecth_feed_items(&link) {
                inserted.extend(insert_new_items(feed, &items));
            };
        }

        if !inserted.is_empty() {
            notify(&app_id, &inserted);
            let _ = handle.emit_all("feed_updated", ());
        }

        thread::sleep(time::Duration::from_secs(120));
    });
}

fn get_links_to_check() -> Vec<(i32, String)> {
    if let Ok(feeds) = feeds::read_all() {
        let current = Utc::now().fixed_offset();
        let filtered = feeds
            .iter()
            .filter(|x| x.checked_at + Duration::seconds(120) <= current)
            .filter(|x| x.status == FeedStatus::Subscribed);

        filtered
            .map(|x| {
                let _ = feeds::update(&FeedToUpdate {
                    id: x.id,
                    title: None,
                    link: None,
                    status: None,
                    checked_at: Some(current),
                });
                (x.id, x.link.clone())
            })
            .collect()
    } else {
        vec![]
    }
}

fn insert_new_items(feed: i32, items: &[RawItem]) -> Vec<ItemToCreate> {
    let current = Utc::now().fixed_offset();

    let args = items.iter().map(|x| ItemToCreate {
        author: x.author.clone().map(|x| x.trim().to_string()),
        title: x.title.trim().to_string(),
        link: x.link.clone().unwrap_or("#".to_string()).trim().to_string(),
        description: x
            .content
            .clone()
            .unwrap_or("".to_string())
            .trim()
            .to_string(),
        status: ItemStatus::Unread,
        published_at: x.published_at.unwrap_or(current),
        feed,
    });

    let mut inserted = vec![];
    for arg in args {
        if items::create(&arg).is_ok() {
            inserted.push(arg);
        }
    }

    inserted
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
