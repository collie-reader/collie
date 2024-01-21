use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Utc};
use rusqlite::Connection;

use crate::models::feeds::FeedStatus;
use crate::syndication::RawItem;
use crate::{
    models::{
        feeds::{self, FeedToUpdate},
        items::{self, ItemOrder, ItemReadOption, ItemStatus, ItemToCreate},
    },
    syndication::fetch_feed_items,
};

pub fn create_new_items(db: &Connection, proxy: Option<&str>) -> Vec<ItemToCreate> {
    let pairs = get_links_to_check(db);

    let most_recent_items = get_most_recent_items(db);

    let mut inserted = vec![];
    for (feed, link, fetch_old_items) in pairs {
        let mut items = fetch_feed_items(&link, proxy).unwrap();

        if !fetch_old_items {
            if let Some(most_recent) = most_recent_items.get(&feed) {
                items.retain(|item| {
                    item.published_at
                        .map_or(false, |published_at| published_at > *most_recent)
                });
            } else {
                items.truncate(1)
            }
        } else {
            items.sort_by_key(|x| x.published_at);
        }

        inserted.extend(insert_new_items(db, feed, &items));
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
                        fetch_old_items: None,
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

fn get_most_recent_items(db: &Connection) -> HashMap<i32, DateTime<FixedOffset>> {
    let opt = ItemReadOption {
        ids: None,
        feed: None,
        status: None,
        is_saved: None,
        order_by: Some(ItemOrder::PublishedDateDesc),
        limit: Some(1),
        offset: None,
    };

    let rows = items::read_all(db, &opt).unwrap();

    let mut most_recent_items = HashMap::new();
    for row in rows {
        let feed = row.id();
        let published_at = row.published_at().unwrap();
        most_recent_items.insert(feed, published_at);
    }

    most_recent_items
}
