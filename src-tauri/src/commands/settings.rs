use tauri::State;

use crate::{
    models::settings::{self, Setting, SettingToUpdate},
    DbState,
};

#[tauri::command]
pub fn read_all_settings(db_state: State<DbState>) -> Result<Vec<Setting>, String> {
    let db = db_state.db.lock().unwrap();
    match settings::read_all(&db) {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_setting(db_state: State<DbState>, arg: SettingToUpdate) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match settings::update(&db, &arg) {
        Ok(_) => Ok("Setting updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
