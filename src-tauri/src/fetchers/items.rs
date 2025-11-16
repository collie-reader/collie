use collie::model::item::{Item, ItemReadOption, ItemToCreate, ItemToUpdate, ItemToUpdateAll};

use super::auth::AuthClient;

pub async fn create(client: &AuthClient, arg: &ItemToCreate) -> Result<String, String> {
    let response = client.post("/items", arg).await?;

    if response.status().is_success() {
        Ok("Item created".to_string())
    } else {
        Err(format!("Failed to create item: {}", response.status()))
    }
}

pub async fn read_all(client: &AuthClient, opt: &ItemReadOption) -> Result<Vec<Item>, String> {
    let response = client.get_with_json("/items", opt).await?;

    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch items: {}", response.status()))
    }
}

pub async fn count_all(client: &AuthClient, opt: &ItemReadOption) -> Result<i64, String> {
    let response = client.get_with_json("/items/count", opt).await?;

    if response.status().is_success() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to count items: {}", response.status()))
    }
}

pub async fn update(client: &AuthClient, arg: &ItemToUpdate) -> Result<String, String> {
    let response = client.patch(&format!("/items/{}", arg.id), arg).await?;

    if response.status().is_success() {
        Ok("Item updated".to_string())
    } else {
        Err(format!("Failed to update item: {}", response.status()))
    }
}

pub async fn update_all(client: &AuthClient, arg: &ItemToUpdateAll) -> Result<String, String> {
    let response = client.patch("/items", arg).await?;

    if response.status().is_success() {
        Ok("Items updated".to_string())
    } else {
        Err(format!("Failed to update items: {}", response.status()))
    }
}
