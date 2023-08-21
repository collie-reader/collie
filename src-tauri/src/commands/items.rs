use crate::models::items::{self, Item, ItemReadOption, ItemToUpdate};

#[tauri::command]
pub fn read_all_items(opt: ItemReadOption) -> Result<Vec<Item>, String> {
    match items::read_all(opt) {
        Ok(items) => Ok(items),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_item(arg: ItemToUpdate) -> Result<String, String> {
    match items::update(arg) {
        Ok(_) => Ok("Item updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
