import { createSignal, For, onMount, Show } from "solid-js";

import "../styles/Settings.css";
import * as api from "../api/settings";

function Settings() {
  const [settings, setSettings] = createSignal<api.Setting[]>([]);
  const [newSettings, setNewSettings] = createSignal<{ [key in api.SettingKey]: string }>({
    [api.SettingKey.POLLING_FREQUENCY]: "",
  });

  const keyToText = (key: api.SettingKey) => {
    switch (key) {
      case api.SettingKey.POLLING_FREQUENCY:
        return "Polling frequency";
    }
  }

  const load = async () => {
    setSettings(await api.readAllSettings());
  };

  const validate = (key: api.SettingKey, value: string) => {
    switch (key) {
      case api.SettingKey.POLLING_FREQUENCY:
        if (Number(value) < 30) {
          return false;
        }
    }

    return true;
  };

  const update = async (key: api.SettingKey, value: string) => {
    switch (key) {
      case api.SettingKey.POLLING_FREQUENCY:
        if (validate(key, value)) {
          setNewSettings({ ...newSettings(), [api.SettingKey.POLLING_FREQUENCY]: "30" })
          return;
        }
    }

    await api.updateSetting({ key, value })
    await load()
  };

  onMount(async () => {
    await load();

    let newSettingsPlaceholder = newSettings();
    settings().forEach((setting: api.Setting) => {
        newSettingsPlaceholder[setting.key] = setting.value;
    });
    setNewSettings({ ...newSettings, ...newSettingsPlaceholder });
  });

  return (
    <div class="container">
      <h2>Settings</h2>
      <ul class="setting-list">
        <For each={settings()}>{(setting) =>
          <li class="row">
            <span><strong>{keyToText(setting.key)}</strong>: Check all feeds every</span>
            <input type="number" min="30" value={newSettings()[setting.key]}
              onInput={(e) => setNewSettings({ ...newSettings(), [setting.key]: e.currentTarget.value })} /> <span>seconds.</span>
            <Show when={validate(setting.key, newSettings()[setting.key]) && newSettings()[setting.key] !== setting.value}>
              <button onClick={() => update(setting.key, newSettings()[setting.key])}>Save</button>
            </Show>
            <small>The seconds cannot be less than 30. A feed that update too quickly may overwhelm you with too many notifications.</small>
          </li>
        }</For>
      </ul>
    </div>
  );
}

export default Settings;
