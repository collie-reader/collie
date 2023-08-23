use tauri::State;

use crate::{
    models::feeds::{self, Feed, FeedToCreate, FeedToUpdate},
    producer::create_new_items,
    syndication::fecth_feed_title,
    DbState,
};

#[tauri::command]
pub fn create_feed(db_state: State<DbState>, arg: FeedToCreate) -> Result<String, String> {
    let title = match fecth_feed_title(&arg.link) {
        Ok(title) => title,
        Err(err) => return Err(err.to_string()),
    };

    let arg = FeedToCreate {
        title,
        link: arg.link,
    };

    let db = db_state.db.lock().unwrap();
    match feeds::create(&db, arg) {
        Ok(_) => {
            let _ = create_new_items(&db);
            Ok("New feed added".to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn read_all_feeds(db_state: State<DbState>) -> Result<Vec<Feed>, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::read_all(&db) {
        Ok(feeds) => Ok(feeds),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn read_feed(db_state: State<DbState>, id: i32) -> Result<Option<Feed>, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::read(&db, id) {
        Ok(feed) => Ok(feed),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_feed(db_state: State<DbState>, arg: FeedToUpdate) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::update(&db, &arg) {
        Ok(_) => Ok("Feed updated".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn delete_feed(db_state: State<DbState>, id: i32) -> Result<String, String> {
    let db = db_state.db.lock().unwrap();
    match feeds::delete(&db, id) {
        Ok(_) => Ok("Feed deleted".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
