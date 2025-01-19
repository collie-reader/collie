use collie::model::feed::{Feed, FeedToCreate, FeedToUpdate};
use collie::repository::database::DbConnection;
use collie::service::feed;
use tauri::State;

#[tauri::command]
pub async fn create_feed(
    state: State<'_, DbConnection>,
    arg: FeedToCreate,
) -> Result<String, String> {
    // TODO: If upstream url set, fetch from the upstream
    match feed::create(&state, &arg, None).await {
        Ok(_) => Ok("New feed added".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn read_all_feeds(state: State<'_, DbConnection>) -> Result<Vec<Feed>, String> {
    match feed::read_all(&state) {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn read_feed(state: State<'_, DbConnection>, id: i32) -> Result<Option<Feed>, String> {
    match feed::read(&state, id) {
        Ok(feed) => Ok(feed),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn update_feed(
    state: State<'_, DbConnection>,
    arg: FeedToUpdate,
) -> Result<String, String> {
    match feed::update(&state, &arg) {
        Ok(_) => Ok("Feed updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn delete_feed(state: State<'_, DbConnection>, id: i32) -> Result<String, String> {
    match feed::delete(&state, id) {
        Ok(_) => Ok("Feed deleted".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
