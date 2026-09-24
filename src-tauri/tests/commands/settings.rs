use tauri::Manager;

use super::test_app;
use crate::commands::settings;
use crate::models::settings::{SettingKey, SettingToUpdate};

#[test]
fn read_all_settings_includes_default_polling_frequency() {
    let app = test_app();

    let settings = settings::read_all_settings(app.state()).unwrap();
    let polling = settings
        .iter()
        .find(|setting| setting.key == SettingKey::PollingFrequency)
        .unwrap();

    assert_eq!(polling.value, "300");
}

#[test]
fn read_setting_returns_default_theme() {
    let app = test_app();

    let theme = settings::read_setting(app.state(), SettingKey::Theme).unwrap();
    assert_eq!(theme.value, "system");
}

#[test]
fn update_setting_persists_theme() {
    let app = test_app();
    let update = SettingToUpdate {
        key: SettingKey::Theme,
        value: "dark".into(),
    };

    settings::update_setting(app.state(), update).unwrap();

    let theme = settings::read_setting(app.state(), SettingKey::Theme).unwrap();
    assert_eq!(theme.value, "dark");
}

#[test]
fn update_setting_rejects_polling_below_minimum() {
    let app = test_app();
    let update = SettingToUpdate {
        key: SettingKey::PollingFrequency,
        value: "29".into(),
    };

    let result = settings::update_setting(app.state(), update);
    assert!(result.is_err());

    let polling = settings::read_setting(app.state(), SettingKey::PollingFrequency).unwrap();
    assert_eq!(polling.value, "300");
}

#[test]
fn update_setting_accepts_minimum_polling_frequency() {
    let app = test_app();
    let update = SettingToUpdate {
        key: SettingKey::PollingFrequency,
        value: "30".into(),
    };

    settings::update_setting(app.state(), update).unwrap();

    let polling = settings::read_setting(app.state(), SettingKey::PollingFrequency).unwrap();
    assert_eq!(polling.value, "30");
}

#[test]
fn update_setting_rejects_schema_version_changes() {
    let app = test_app();
    let update = SettingToUpdate {
        key: SettingKey::DbSchemeVersion,
        value: "2".into(),
    };

    let result = settings::update_setting(app.state(), update);
    assert_eq!(result.unwrap_err(), "forbidden");

    let version = settings::read_setting(app.state(), SettingKey::DbSchemeVersion).unwrap();
    assert_eq!(version.value, "1");
}

#[test]
fn update_setting_adds_https_to_upstream_url() {
    let app = test_app();
    let update = SettingToUpdate {
        key: SettingKey::UpstreamUrl,
        value: "example.test".into(),
    };

    settings::update_setting(app.state(), update).unwrap();

    let url = settings::read_setting(app.state(), SettingKey::UpstreamUrl).unwrap();
    assert_eq!(url.value, "https://example.test");
}

#[test]
fn update_setting_preserves_explicit_http_scheme() {
    let app = test_app();
    let update = SettingToUpdate {
        key: SettingKey::UpstreamUrl,
        value: "http://example.test".into(),
    };

    settings::update_setting(app.state(), update).unwrap();

    let url = settings::read_setting(app.state(), SettingKey::UpstreamUrl).unwrap();
    assert_eq!(url.value, "http://example.test");
}

#[test]
fn update_setting_allows_disabling_upstream() {
    let app = test_app();
    super::enable_upstream_without_credentials(&app);

    let update = SettingToUpdate {
        key: SettingKey::UpstreamUrl,
        value: String::new(),
    };

    settings::update_setting(app.state(), update).unwrap();

    let url = settings::read_setting(app.state(), SettingKey::UpstreamUrl).unwrap();
    assert_eq!(url.value, "");
}
