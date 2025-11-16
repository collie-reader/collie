use collie::model::feed::{Feed, FeedToCreate, FeedToUpdate};
use collie::repository::database::DbConnection;
use collie::service::feed;
use tauri::State;

use crate::fetchers;
use crate::fetchers::auth::AuthClient;
use crate::models::settings;

fn create_auth_client(state: &DbConnection, url: String) -> Result<AuthClient, String> {
    let (access, secret) = settings::upstream_credentials(state)
        .ok_or_else(|| "Upstream credentials not configured".to_string())?;
    Ok(AuthClient::new(url, access, secret))
}

#[tauri::command]
pub async fn create_feed(
    state: State<'_, DbConnection>,
    arg: FeedToCreate,
) -> Result<String, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::feeds::create(&client, &arg).await
        }
        None => match feed::create(&state, &arg, None).await {
            Ok(_) => Ok("New feed added".to_string()),
            Err(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
pub async fn read_all_feeds(state: State<'_, DbConnection>) -> Result<Vec<Feed>, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::feeds::read_all(&client).await
        }
        None => match feed::read_all(&state) {
            Ok(feeds) => Ok(feeds),
            Err(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
pub async fn read_feed(state: State<'_, DbConnection>, id: i32) -> Result<Option<Feed>, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::feeds::read(&client, id).await
        }
        None => match feed::read(&state, id) {
            Ok(feed) => Ok(feed),
            Err(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
pub async fn update_feed(
    state: State<'_, DbConnection>,
    arg: FeedToUpdate,
) -> Result<String, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::feeds::update(&client, &arg).await
        }
        None => match feed::update(&state, &arg) {
            Ok(_) => Ok("Feed updated".to_string()),
            Err(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
pub async fn delete_feed(state: State<'_, DbConnection>, id: i32) -> Result<String, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::feeds::delete(&client, id).await
        }
        None => match feed::delete(&state, id) {
            Ok(_) => Ok("Feed deleted".to_string()),
            Err(err) => Err(err.to_string()),
        },
    }
}
