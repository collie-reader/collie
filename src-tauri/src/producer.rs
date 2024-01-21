use chrono::Utc;
use rusqlite::Connection;

use crate::models::feeds::FeedStatus;
use crate::syndication::RawItem;
use crate::{
    models::{
        feeds::{self, FeedToUpdate},
        items::{self, ItemStatus, ItemToCreate},
    },
    syndication::fetch_feed_items,
};

pub fn create_new_items(db: &Connection, proxy: Option<&str>) -> Vec<ItemToCreate> {
    let pairs = get_links_to_check(db);

    let mut inserted = vec![];
    for (feed, link, fetch_old_items) in pairs {
        if let Ok(mut items) = fetch_feed_items(&link, proxy, fetch_old_items) {
            items.sort_by_key(|x| x.published_at);
            inserted.extend(insert_new_items(db, feed, &items));
        };
    }

    inserted
}

fn get_links_to_check(db: &Connection) -> Vec<(i32, String, bool)> {
    if let Ok(feeds) = feeds::read_all(db) {
        let current = Utc::now().fixed_offset();
        let filtered = feeds.iter().filter(|x| x.status == FeedStatus::Subscribed);

        filtered
            .map(|x| {
                let _ = feeds::update(
                    db,
                    &(FeedToUpdate {
                        id: x.id,
                        title: None,
                        link: None,
                        status: None,
                        checked_at: Some(current),
                        fetch_old_items: Some(x.fetch_old_items),
                    }),
                );
                (x.id, x.link.clone(), x.fetch_old_items)
            })
            .collect()
    } else {
        vec![]
    }
}

fn insert_new_items(db: &Connection, feed: i32, items: &[RawItem]) -> Vec<ItemToCreate> {
    let current = Utc::now().fixed_offset();

    let args = items.iter().map(|x| ItemToCreate {
        author: x.author.clone().map(|x| x.trim().to_string()),
        title: x.title.trim().to_string(),
        link: x.link.clone().unwrap_or("#".to_string()).trim().to_string(),
        description: x
            .content
            .clone()
            .unwrap_or(String::new())
            .trim()
            .to_string(),
        status: ItemStatus::Unread,
        published_at: x.published_at.unwrap_or(current),
        feed,
    });

    let mut inserted = vec![];
    for arg in args {
        if items::create(db, &arg).is_ok() {
            inserted.push(arg);
        }
    }

    inserted
}
