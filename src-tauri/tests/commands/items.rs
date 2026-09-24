use collie::model::item::{ItemOrder, ItemReadOption, ItemStatus, ItemToUpdate, ItemToUpdateAll};
use tauri::Manager;

use super::{enable_upstream_without_credentials, item_options, seed_feed, seed_item, test_app};
use crate::commands::items;

#[tokio::test]
async fn read_all_items_filters_by_feed_and_status() {
    let app = test_app();

    let feed_id = seed_feed(&app, "target");
    let other_feed_id = seed_feed(&app, "other");
    let unread_id = seed_item(&app, feed_id, "unread", ItemStatus::Unread);

    seed_item(&app, feed_id, "read", ItemStatus::Read);
    seed_item(&app, other_feed_id, "other", ItemStatus::Unread);

    let options = ItemReadOption {
        feed: Some(feed_id),
        status: Some(ItemStatus::Unread),
        ..item_options()
    };

    let items = items::read_all_items(app.state(), app.state(), options)
        .await
        .unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, unread_id);
}

#[tokio::test]
async fn read_all_items_returns_requested_page() {
    let app = test_app();

    let feed_id = seed_feed(&app, "example");
    let older_id = seed_item(&app, feed_id, "older", ItemStatus::Unread);

    seed_item(&app, feed_id, "newer", ItemStatus::Unread);

    let options = ItemReadOption {
        order_by: Some(ItemOrder::ReceivedDateDesc),
        limit: Some(1),
        offset: Some(1),
        ..item_options()
    };

    let page = items::read_all_items(app.state(), app.state(), options)
        .await
        .unwrap();

    assert_eq!(page.len(), 1);
    assert_eq!(page[0].id, older_id);
}

#[tokio::test]
async fn count_all_items_counts_only_matching_status() {
    let app = test_app();

    let feed_id = seed_feed(&app, "example");
    seed_item(&app, feed_id, "unread", ItemStatus::Unread);
    seed_item(&app, feed_id, "read", ItemStatus::Read);

    let options = ItemReadOption {
        status: Some(ItemStatus::Unread),
        ..item_options()
    };

    let count = items::count_all_items(app.state(), app.state(), options)
        .await
        .unwrap();

    assert_eq!(count, 1);
}

#[tokio::test]
async fn update_item_saves_without_changing_read_status() {
    let app = test_app();

    let feed_id = seed_feed(&app, "example");
    let item_id = seed_item(&app, feed_id, "article", ItemStatus::Unread);

    let update = ItemToUpdate {
        id: item_id,
        status: None,
        is_saved: Some(true),
    };

    items::update_item(app.state(), app.state(), update)
        .await
        .unwrap();

    let items = items::read_all_items(app.state(), app.state(), item_options())
        .await
        .unwrap();

    assert!(items[0].is_saved);
    assert!(matches!(items[0].status, ItemStatus::Unread));
}

#[tokio::test]
async fn update_items_marks_only_selected_items_as_read() {
    let app = test_app();

    let feed_id = seed_feed(&app, "example");
    let first_id = seed_item(&app, feed_id, "first", ItemStatus::Unread);
    let second_id = seed_item(&app, feed_id, "second", ItemStatus::Unread);

    seed_item(&app, feed_id, "other", ItemStatus::Unread);

    let update = ItemToUpdateAll {
        status: Some(ItemStatus::Read),
        is_saved: None,
        opt: Some(ItemReadOption {
            ids: Some(vec![first_id, second_id]),
            ..item_options()
        }),
    };

    items::update_items(app.state(), app.state(), update)
        .await
        .unwrap();

    let mut items = items::read_all_items(app.state(), app.state(), item_options())
        .await
        .unwrap();

    items.sort_by_key(|item| item.id);

    assert_eq!(items.len(), 3);
    assert!(matches!(items[0].status, ItemStatus::Read));
    assert!(matches!(items[1].status, ItemStatus::Read));
    assert!(matches!(items[2].status, ItemStatus::Unread));
}

#[tokio::test]
async fn read_all_items_requires_upstream_credentials() {
    let app = test_app();
    enable_upstream_without_credentials(&app);

    let result = items::read_all_items(app.state(), app.state(), item_options()).await;
    assert_eq!(result.unwrap_err(), "Upstream credentials not configured");
}

#[tokio::test]
async fn update_item_requires_upstream_credentials() {
    let app = test_app();
    enable_upstream_without_credentials(&app);

    let update = ItemToUpdate {
        id: 1,
        status: Some(ItemStatus::Read),
        is_saved: None,
    };

    let result = items::update_item(app.state(), app.state(), update).await;
    assert_eq!(result.unwrap_err(), "Upstream credentials not configured");
}
