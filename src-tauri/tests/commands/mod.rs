mod feeds;
mod items;
mod settings;

use collie::model::{
    feed::FeedToCreate,
    item::{ItemReadOption, ItemStatus, ItemToCreate},
};
use collie::repository::{database, database::DbConnection, feed, item};
use std::sync::{Arc, Mutex};
use tauri::{test::MockRuntime, App, Manager};

use crate::{fetchers::auth::AuthClientProvider, models};

fn test_app() -> App<MockRuntime> {
    let db = rusqlite::Connection::open_in_memory().unwrap();

    database::Migration::new()
        .table(database::feeds_table())
        .table(database::items_table())
        .table(models::database::settings_table())
        .migrate(&db)
        .unwrap();

    models::database::insert_default_settings(&db).unwrap();

    let conn: DbConnection = Arc::new(Mutex::new(db));
    tauri::test::mock_builder()
        .manage(conn)
        .manage(AuthClientProvider::new().unwrap())
        .build(tauri::test::mock_context(tauri::test::noop_assets()))
        .unwrap()
}

fn seed_feed(app: &App<MockRuntime>, title: &str) -> i32 {
    let conn = app.state::<DbConnection>();

    feed::create(
        &conn,
        &FeedToCreate {
            title: title.into(),
            link: format!("https://example.test/{title}.xml"),
            fetch_old_items: true,
        },
    )
    .unwrap();

    let id = conn.lock().unwrap().last_insert_rowid() as i32;

    id
}

fn seed_item(app: &App<MockRuntime>, feed: i32, title: &str, status: ItemStatus) -> i32 {
    let conn = app.state::<DbConnection>();

    item::create(
        &conn,
        &ItemToCreate {
            author: None,
            title: title.into(),
            description: "Test body".into(),
            link: format!("https://example.test/{title}"),
            status,
            published_at: "2026-01-01T00:00:00Z".parse().unwrap(),
            feed,
        },
    )
    .unwrap();

    let id = conn.lock().unwrap().last_insert_rowid() as i32;

    id
}

fn item_options() -> ItemReadOption {
    ItemReadOption {
        ids: None,
        feed: None,
        status: None,
        is_saved: None,
        order_by: None,
        limit: None,
        offset: None,
    }
}

fn enable_upstream_without_credentials(app: &App<MockRuntime>) {
    crate::commands::settings::update_setting(
        app.state(),
        models::settings::SettingToUpdate {
            key: models::settings::SettingKey::UpstreamUrl,
            value: "https://example.test".into(),
        },
    )
    .unwrap();
}
