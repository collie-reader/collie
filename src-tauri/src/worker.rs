use std::thread;
use std::time;
use tauri::AppHandle;
use tauri::Manager;

use chrono::{DateTime, Duration, Utc};
use tauri::api::notification::Notification;

use crate::{
    models::{
        feeds::{self, FeedToUpdate},
        items::{self, ItemStatus, ItemToCreate},
    },
    rss::fecth_feed_channel,
};

pub fn start(app_id: String, handle: AppHandle) {
    thread::spawn(move || loop {
        let pairs = get_links_to_check();

        let mut inserted = vec![];
        for (feed, link) in pairs {
            if let Ok(channel) = fecth_feed_channel(&link) {
                inserted.extend(insert_unread_items(feed, channel.items()));
            };
        }

        if inserted.len() > 0 {
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
            .filter(|x| x.checked_at + Duration::seconds(120) <= current);

        filtered
            .map(|x| {
                let _ = feeds::update(&FeedToUpdate {
                    id: x.id,
                    title: None,
                    link: None,
                    checked_at: Some(current),
                });
                (x.id, x.link.clone())
            })
            .collect()
    } else {
        vec![]
    }
}

fn insert_unread_items(feed: i32, items: &[rss::Item]) -> Vec<ItemToCreate> {
    let current = Utc::now().fixed_offset();

    let args = items.iter().map(|x| ItemToCreate {
        author: x
            .author()
            .map(|x| x.trim().to_string())
            .or(x.dublin_core_ext().map(|x| x.creators().join(", "))),
        title: x.title().unwrap_or("Untitled").trim().to_string(),
        link: x.link().unwrap_or("#").trim().to_string(),
        description: x.description().unwrap_or("").trim().to_string(),
        status: ItemStatus::Unread,
        published_at: DateTime::parse_from_rfc2822(x.pub_date().unwrap_or(&current.to_rfc2822()))
            .unwrap_or(Utc::now().fixed_offset()),
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
