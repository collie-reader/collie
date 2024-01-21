use tauri::State;

use crate::models::settings;
use crate::models::settings::SettingKey;
use crate::{
    models::feeds::{self, Feed, FeedToCreate, FeedToUpdate},
    producer::create_new_items,
    syndication::{fetch_content, fetch_feed_title, find_feed_link, is_feed},
    DbState,
};

#[tauri::command]
pub fn create_feed(db_state: State<DbState>, arg: FeedToCreate) -> Result<String, String> {
    if arg.link.is_empty() {
        return Err("Link cannot be empty".to_string());
    }

    let db = db_state.db.lock().unwrap();
    let proxy = settings::read(&db, &SettingKey::Proxy)
        .map(|x| x.value)
        .ok();

    let link = if is_feed(&arg.link, proxy.as_deref()).unwrap() {
        arg.link.clone()
    } else {
        let html_content = fetch_content(&arg.link, proxy.as_deref()).unwrap();
        // TODO: Notification if RSS address is converted?
        if let Some(rss_link) = find_feed_link(&html_content).unwrap() {
            rss_link
        } else {
            // TODO: Notification when RSS feeds cannot be fetched?
            return Err("No valid RSS/Atom link found".to_string());
        }
    };

    let title = match fetch_feed_title(&link, proxy.as_deref()) {
        Ok(title) => title,
        Err(err) => return Err(err.to_string()),
    };

    let arg = FeedToCreate {
        title,
        link,
        fetch_old_items: arg.fetch_old_items,
    };

    match feeds::create(&db, &arg) {
        Ok(_) => {
            let _ = create_new_items(&db, proxy.as_deref());
            Ok("New feed added".to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn read_all_feeds(db_state: State<DbState>) -> Result<Vec<Feed>, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::read_all(&db) {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn read_feed(db_state: State<DbState>, id: i32) -> Result<Option<Feed>, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::read(&db, id) {
        Ok(feed) => Ok(feed),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_feed(db_state: State<DbState>, arg: FeedToUpdate) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::update(&db, &arg) {
        Ok(_) => Ok("Feed updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn delete_feed(db_state: State<DbState>, id: i32) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::delete(&db, id) {
        Ok(_) => Ok("Feed deleted".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
