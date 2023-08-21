use std::error::Error;

use rss::Channel;

use crate::models::feeds::{self, Feed, FeedToCreate, FeedToUpdate};

#[tauri::command]
pub async fn create_feed(arg: FeedToCreate) -> Result<String, String> {
    let title = match fecth_feed_channel(&arg.link).await {
        Ok(channel) => channel.title,
        Err(err) => return Err(err.to_string()),
    };

    let arg = FeedToCreate {
        title,
        link: arg.link.to_string(),
    };

    match feeds::create(arg) {
        Ok(_) => Ok("New feed added".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn read_all_feeds() -> Result<Vec<Feed>, String> {
    match feeds::read_all() {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn update_feed(arg: FeedToUpdate) -> Result<String, String> {
    match feeds::update(arg) {
        Ok(_) => Ok("Feed updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn delete_feed(id: i32) -> Result<String, String> {
    match feeds::delete(id) {
        Ok(_) => Ok("Feed deleted".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

async fn fecth_feed_channel(link: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(link).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
