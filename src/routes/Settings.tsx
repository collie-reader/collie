import { createSignal, For, Match, onMount, Show, Switch } from "solid-js";

import "../styles/Settings.css";
import * as api from "../api/settings";

function Settings() {
  const [settings, setSettings] = createSignal<api.Setting[]>([]);
  const [newSettings, setNewSettings] = createSignal<{ [key in api.SettingKey]: string }>({
    [api.SettingKey.POLLING_FREQUENCY]: "",
    [api.SettingKey.NOTIFICATION]: "",
    [api.SettingKey.DB_SCHEME_VERSION]: "",
    [api.SettingKey.THEME]: "",
  });

  const keyToText = (key: api.SettingKey) => {
    switch (key) {
      case api.SettingKey.POLLING_FREQUENCY:
        return "Polling frequency";
      case api.SettingKey.NOTIFICATION:
        return "Notification";
      case api.SettingKey.THEME:
        return "Theme";
      default:
        return "";
    }
  }

  const load = async () => {
    setSettings(await api.readAllSettings());
  };

  const validate = (key: api.SettingKey, value: string) => {
    switch (key) {
      case api.SettingKey.POLLING_FREQUENCY:
        if (Number(value) < 30) return false;
        break;
      case api.SettingKey.NOTIFICATION:
        if (value !== "1" && value !== "0") return false;
    }

    return true;
  };

  const update = async (key: api.SettingKey, value: string) => {
    switch (key) {
      case api.SettingKey.POLLING_FREQUENCY:
        if (!validate(key, value)) {
          setNewSettings({ ...newSettings(), [api.SettingKey.POLLING_FREQUENCY]: "30" })
          return;
        }
    }

    await api.updateSetting({ key, value })
    await load()
  };

  const SaveButton = (setting: api.Setting, afterUpdate: () => void = () => {}) =>
    <Show when={validate(setting.key, newSettings()[setting.key]) && newSettings()[setting.key] !== setting.value}>
      <button onClick={() => {
        update(setting.key, newSettings()[setting.key]);
        afterUpdate();
      }}>Save</button>
    </Show>;

  onMount(async () => {
    await load();

    let newSettingsPlaceholder = newSettings();
    settings().forEach((setting: api.Setting) => {
        newSettingsPlaceholder[setting.key] = setting.value;
    });
    setNewSettings({ ...newSettings, ...newSettingsPlaceholder });
  });

  return (
    <div class="settings-page container">
      <h2>Settings</h2>
      <ul class="setting-list">
        <For each={settings()}>{(setting) =>
          <li class="row">
            <Switch>
              <Match when={setting.key === api.SettingKey.POLLING_FREQUENCY}>
                <span><strong>{keyToText(setting.key)}</strong>: Check all feeds every</span>
                <input type="number" min="30" value={newSettings()[setting.key]}
                  onInput={(e) => setNewSettings({ ...newSettings(), [setting.key]: e.currentTarget.value })} /> <span>seconds.</span>
                {SaveButton(setting)}
                <small>The seconds cannot be less than 30. A feed that update too quickly may overwhelm you.</small>
              </Match>
              <Match when={setting.key === api.SettingKey.NOTIFICATION}>
                <span><strong>{keyToText(setting.key)}</strong>: Do you want to be notified when new items are arrived?</span>
                <label for="yes"><input type="radio" id="yes" name={setting.key} value="1"
                  checked={newSettings()[setting.key] === "1"}
                  onChange={(e) => setNewSettings({ ...newSettings(), [setting.key]: e.currentTarget.value })} />Yes</label>
                <label for="no"><input type="radio" id="no" name={setting.key} value="0"
                  checked={newSettings()[setting.key] === "0"}
                  onChange={(e) => setNewSettings({ ...newSettings(), [setting.key]: e.currentTarget.value })} />No</label>
                {SaveButton(setting)}
              </Match>
              <Match when={setting.key === api.SettingKey.THEME}>
                <span><strong>{keyToText(setting.key)}</strong>: </span>
                <label for="system"><input type="radio" id="system" name={setting.key} value="system"
                  checked={newSettings()[setting.key] === "system"}
                  onChange={(e) => setNewSettings({ ...newSettings(), [setting.key]: e.currentTarget.value })} />Sync with system</label>
                <label for="light"><input type="radio" id="light" name={setting.key} value="light"
                  checked={newSettings()[setting.key] === "light"}
                  onChange={(e) => setNewSettings({ ...newSettings(), [setting.key]: e.currentTarget.value })} />Light</label>
                <label for="dark"><input type="radio" id="dark" name={setting.key} value="dark"
                  checked={newSettings()[setting.key] === "dark"}
                  onChange={(e) => setNewSettings({ ...newSettings(), [setting.key]: e.currentTarget.value })} />Dark</label>
                {SaveButton(setting, () => location.reload())}
              </Match>
            </Switch>
          </li>
        }</For>
      </ul>
    </div>
  );
}

export default Settings;
