use collie::model::feed::{FeedToCreate, FeedToUpdate};
use tauri::Manager;

use super::{enable_upstream_without_credentials, seed_feed, test_app};
use crate::commands::feeds;

#[tokio::test]
async fn read_all_feeds_returns_stored_feeds() {
    let app = test_app();

    let feed_id = seed_feed(&app, "example");
    let feeds = feeds::read_all_feeds(app.state(), app.state())
        .await
        .unwrap();

    assert_eq!(feeds.len(), 1);
    assert_eq!(feeds[0].id, feed_id);
}

#[tokio::test]
async fn update_feed_changes_title() {
    let app = test_app();

    let feed_id = seed_feed(&app, "original");
    let update = FeedToUpdate {
        id: feed_id,
        title: Some("Renamed".into()),
        link: None,
        status: None,
        checked_at: None,
        fetch_old_items: None,
    };

    feeds::update_feed(app.state(), app.state(), update)
        .await
        .unwrap();

    let feed = feeds::read_feed(app.state(), app.state(), feed_id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(feed.title, "Renamed");
}

#[tokio::test]
async fn delete_feed_removes_only_the_target() {
    let app = test_app();

    let target_id = seed_feed(&app, "target");
    let other_id = seed_feed(&app, "other");

    feeds::delete_feed(app.state(), app.state(), target_id)
        .await
        .unwrap();

    let remaining = feeds::read_all_feeds(app.state(), app.state())
        .await
        .unwrap();

    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].id, other_id);
}

#[tokio::test]
async fn create_feed_rejects_empty_url() {
    let app = test_app();

    let feed = FeedToCreate {
        title: "Example".into(),
        link: String::new(),
        fetch_old_items: true,
    };

    let result = feeds::create_feed(app.state(), app.state(), feed).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn read_all_feeds_requires_upstream_credentials() {
    let app = test_app();
    enable_upstream_without_credentials(&app);

    let result = feeds::read_all_feeds(app.state(), app.state()).await;

    assert_eq!(result.unwrap_err(), "Upstream credentials not configured");
}
