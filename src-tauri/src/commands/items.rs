use collie::{
    model::item::{Item, ItemReadOption, ItemToUpdate, ItemToUpdateAll},
    repository::database::DbConnection,
    service::item,
};
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
pub async fn read_all_items(
    state: State<'_, DbConnection>,
    opt: ItemReadOption,
) -> Result<Vec<Item>, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::items::read_all(&client, &opt).await
        }
        None => match item::read_all(&state, &opt) {
            Ok(items) => Ok(items),
            Err(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
pub async fn count_all_items(
    state: State<'_, DbConnection>,
    opt: ItemReadOption,
) -> Result<i64, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::items::count_all(&client, &opt).await
        }
        None => match item::count_all(&state, &opt) {
            Ok(count) => Ok(count),
            Err(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
pub async fn update_item(
    state: State<'_, DbConnection>,
    arg: ItemToUpdate,
) -> Result<String, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::items::update(&client, &arg).await
        }
        None => match item::update(&state, &arg) {
            Ok(_) => Ok("Item updated".to_string()),
            Err(err) => Err(err.to_string()),
        },
    }
}

#[tauri::command]
pub async fn update_items(
    state: State<'_, DbConnection>,
    arg: ItemToUpdateAll,
) -> Result<String, String> {
    match settings::upstream_url(&state) {
        Some(url) => {
            let client = create_auth_client(&state, url)?;
            fetchers::items::update_all(&client, &arg).await
        }
        None => match item::update_all(&state, &arg) {
            Ok(_) => Ok("Items updated".to_string()),
            Err(err) => Err(err.to_string()),
        },
    }
}
