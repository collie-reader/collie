use collie::model::database::DbConnection;
use collie::model::feed::{self, Feed, FeedToCreate, FeedToUpdate};
use collie::producer::syndication::Feed as SyndicationFeed;
use collie::producer::syndication::{fetch_content, fetch_feed_title, find_feed_link};
use collie::producer::worker::create_new_items;
use std::sync::Arc;
use tauri::State;

use crate::models::settings;
use crate::models::settings::SettingKey;

use crate::error::Error;

#[tauri::command]
pub async fn create_feed(
    state: State<'_, Arc<DbConnection>>,
    arg: FeedToCreate,
) -> Result<String, String> {
    if arg.link.is_empty() {
        return Err(Error::EmptyString.to_string());
    }

    let proxy = settings::read(&state, &SettingKey::Proxy)
        .map(|x| x.value)
        .ok();

    let html_content = fetch_content(&arg.link, proxy.as_deref()).await.unwrap();
    let is_feed = html_content.parse::<SyndicationFeed>().is_ok();

    let link = if is_feed {
        arg.link.clone()
    } else if let Some(rss_link) = find_feed_link(&html_content).unwrap() {
        rss_link
    } else {
        return Err(Error::InvalidFeedLink(arg.link).to_string());
    };

    let title = match fetch_feed_title(&link, proxy.as_deref()).await {
        Ok(title) => title,
        Err(err) => return Err(err.to_string()),
    };

    let arg = FeedToCreate {
        title,
        link,
        fetch_old_items: arg.fetch_old_items,
    };

    match feed::create(&state, &arg) {
        Ok(_) => {
            let _ = create_new_items(&state, proxy.as_deref()).await;
            Ok("New feed added".to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn read_all_feeds(state: State<'_, Arc<DbConnection>>) -> Result<Vec<Feed>, String> {
    match feed::read_all(&state) {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn read_feed(
    state: State<'_, Arc<DbConnection>>,
    id: i32,
) -> Result<Option<Feed>, String> {
    match feed::read(&state, id) {
        Ok(feed) => Ok(feed),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn update_feed(
    state: State<'_, Arc<DbConnection>>,
    arg: FeedToUpdate,
) -> Result<String, String> {
    match feed::update(&state, &arg) {
        Ok(_) => Ok("Feed updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn delete_feed(state: State<'_, Arc<DbConnection>>, id: i32) -> Result<String, String> {
    match feed::delete(&state, id) {
        Ok(_) => Ok("Feed deleted".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
