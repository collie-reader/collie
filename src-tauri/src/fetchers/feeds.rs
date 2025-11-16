use collie::model::feed::{Feed, FeedToCreate, FeedToUpdate};

use super::auth::AuthClient;

pub async fn create(client: &AuthClient, arg: &FeedToCreate) -> Result<String, String> {
    let response = client.post("/feeds", arg).await?;

    if response.status().is_success() {
        Ok("New feed added".to_string())
    } else {
        Err(format!("Failed to create feed: {}", response.status()))
    }
}

pub async fn read_all(client: &AuthClient) -> Result<Vec<Feed>, String> {
    let response = client.get("/feeds").await?;

    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch feeds: {}", response.status()))
    }
}

pub async fn read(client: &AuthClient, id: i32) -> Result<Option<Feed>, String> {
    let response = client.get(&format!("/feeds/{}", id)).await?;

    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch feed: {}", response.status()))
    }
}

pub async fn update(client: &AuthClient, arg: &FeedToUpdate) -> Result<String, String> {
    let response = client.patch(&format!("/feeds/{}", arg.id), arg).await?;

    if response.status().is_success() {
        Ok("Feed updated".to_string())
    } else {
        Err(format!("Failed to update feed: {}", response.status()))
    }
}

pub async fn delete(client: &AuthClient, id: i32) -> Result<String, String> {
    let response = client.delete(&format!("/feeds/{}", id)).await?;

    if response.status().is_success() {
        Ok("Feed deleted".to_string())
    } else {
        Err(format!("Failed to delete feed: {}", response.status()))
    }
}
