use collie::{
    model::item::{Item, ItemReadOption, ItemToUpdate, ItemToUpdateAll},
    repository::database::DbConnection,
    service::item,
};
use tauri::State;

#[tauri::command]
pub fn read_all_items(
    state: State<'_, DbConnection>,
    opt: ItemReadOption,
) -> Result<Vec<Item>, String> {
    match item::read_all(&state, &opt) {
        Ok(items) => Ok(items),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn count_all_items(state: State<'_, DbConnection>, opt: ItemReadOption) -> Result<i64, String> {
    match item::count_all(&state, &opt) {
        Ok(count) => Ok(count),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_item(state: State<'_, DbConnection>, arg: ItemToUpdate) -> Result<String, String> {
    match item::update(&state, &arg) {
        Ok(_) => Ok("Item updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_items(
    state: State<'_, DbConnection>,
    arg: ItemToUpdateAll,
) -> Result<String, String> {
    match item::update_all(&state, &arg) {
        Ok(_) => Ok("Items updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
