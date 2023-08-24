use tauri::State;

use crate::{
    models::items::{self, Item, ItemReadOption, ItemToUpdate, ItemToUpdateAll},
    DbState,
};

#[tauri::command]
pub fn read_all_items(db_state: State<DbState>, opt: ItemReadOption) -> Result<Vec<Item>, String> {
    let db = db_state.db.lock().unwrap();
    match items::read_all(&db, opt) {
        Ok(items) => Ok(items),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn count_all_items(db_state: State<DbState>, opt: ItemReadOption) -> Result<i64, String> {
    let db = db_state.db.lock().unwrap();
    match items::count_all(&db, opt) {
        Ok(count) => Ok(count),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_item(db_state: State<DbState>, arg: ItemToUpdate) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match items::update(&db, arg) {
        Ok(_) => Ok("Item updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_items(db_state: State<DbState>, arg: ItemToUpdateAll) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match items::update_all(&db, arg) {
        Ok(_) => Ok("Items updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
