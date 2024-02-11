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

use crate::error::{Error, Result};

pub fn create_new_items(db: &Connection, proxy: Option<&str>) -> Result<Vec<ItemToCreate>> {
    let pairs = get_links_to_check(db);

    let mut inserted = vec![];

    let mut most_recent_items: Option<HashMap<i32, DateTime<FixedOffset>>> = None;

    for (feed, link, fetch_old_items) in pairs {
        if !fetch_old_items {
            if most_recent_items.is_none() {
                most_recent_items = match get_most_recent_items(db) {
                    Ok(items) => Some(items),
                    Err(err) => return Err(Error::FetchFeedItemsFailure(err.to_string())),
                };
            }
        }

        match fetch_feed_items(&link, proxy) {
            Ok(mut items) => {
                if let Some(ref most_recent) = most_recent_items {
                    if let Some(most_recent) = most_recent.get(&feed) {
                        items.retain(|item| {
                            item.published_at
                                .map_or(false, |published_at| published_at > *most_recent)
                        });
                    } else {
                        items.truncate(1)
                    }
                }

                items.sort_by_key(|x| x.published_at);
                inserted.extend(insert_new_items(db, feed, &items));
            }
            Err(err) => return Err(Error::FetchFeedItemsFailure(err.to_string())),
        }
    }

    Ok(inserted)
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

fn get_most_recent_items(db: &Connection) -> Result<HashMap<i32, DateTime<FixedOffset>>> {
    let mut most_recent_items = HashMap::new();

    let feed_ids = get_all_feed_ids(db)?;

    for feed_id in feed_ids {
        let opt = ItemReadOption {
            ids: None,
            feed: Some(feed_id),
            status: None,
            is_saved: None,
            order_by: Some(ItemOrder::PublishedDateDesc),
            limit: Some(1),
            offset: None,
        };

        match items::read_all(db, &opt) {
            Ok(items) => {
                if let Some(item) = items.first() {
                    most_recent_items.insert(item.feed.id, item.published_at);
                }
            }
            Err(err) => return Err(Error::InvalidValue(err.to_string())),
        }
    }

    Ok(most_recent_items)
}

fn get_all_feed_ids(db: &Connection) -> Result<Vec<i32>> {
    match feeds::read_all(db) {
        Ok(feeds) => Ok(feeds.iter().map(|x| x.id).collect()),
        Err(err) => Err(Error::FetchFeedFailure(err.to_string())),
    }
}
