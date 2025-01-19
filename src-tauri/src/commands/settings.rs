use crate::models::settings::{self, Setting, SettingKey, SettingToUpdate};
use collie::repository::database::DbConnection;
use tauri::State;

#[tauri::command]
pub fn read_all_settings(state: State<'_, DbConnection>) -> Result<Vec<Setting>, String> {
    match settings::read_all(&state) {
        Ok(settings) => Ok(settings),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn read_setting(state: State<'_, DbConnection>, key: SettingKey) -> Result<Setting, String> {
    match settings::read(&state, &key) {
        Ok(setting) => Ok(setting),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_setting(
    state: State<'_, DbConnection>,
    arg: SettingToUpdate,
) -> Result<String, String> {
    match settings::update(&state, &arg) {
        Ok(_) => Ok("Setting updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
