use std::error::Error;

use rss::Channel;

use crate::models::feeds::{self, FeedToCreate};

#[tauri::command]
pub async fn create_feed(link: &str) -> Result<String, String> {
    let title = match fecth_feed_channel(link).await {
        Ok(channel) => channel.title,
        Err(err) => return Err(err.to_string()),
    };

    let args = FeedToCreate {
        title,
        link: link.to_string(),
    };

    match feeds::create(args) {
        Ok(_) => Ok("New feed added".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

async fn fecth_feed_channel(link: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(link).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
