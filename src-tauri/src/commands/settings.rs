use crate::models::settings::{self, Setting, SettingToUpdate};

#[tauri::command]
pub fn read_all_settings() -> Result<Vec<Setting>, String> {
    match settings::read_all() {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_setting(arg: SettingToUpdate) -> Result<String, String> {
    match settings::update(&arg) {
        Ok(_) => Ok("Setting updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
