use std::thread;
use std::time;

use chrono::{DateTime, Duration, Utc};

use crate::{
    models::{
        feeds::{self, FeedToUpdate},
        items::{self, ItemStatus, ItemToCreate},
    },
    rss::fecth_feed_channel,
};

pub fn start() {
    thread::spawn(|| loop {
        let pairs = get_links_to_check();

        for (feed, link) in pairs {
            if let Ok(channel) = fecth_feed_channel(&link) {
                insert_unread_items(feed, channel.items());
            };
        }

        thread::sleep(time::Duration::from_secs(300));
    });
}

pub fn get_links_to_check() -> Vec<(i32, String)> {
    if let Ok(feeds) = feeds::read_all() {
        let current = Utc::now().fixed_offset();
        let filtered = feeds
            .iter()
            .filter(|x| x.checked_at + Duration::seconds(300) <= current);

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

pub fn insert_unread_items(feed: i32, items: &[rss::Item]) {
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

    for arg in args {
        let _ = items::create(arg);
    }
}
