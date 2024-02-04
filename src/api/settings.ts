import { invoke } from "@tauri-apps/api/tauri";

export enum SettingKey {
  POLLING_FREQUENCY = "PollingFrequency",
  NOTIFICATION = "Notification",
  DB_SCHEME_VERSION = "DbSchemeVersion",
  THEME = "Theme",
  ITEMS_ORDER = "ItemsOrder",
  PROXY="Proxy",
  FETCH_OLD_ITEMS = "FetchOldItems"
}

export interface Setting {
  key: SettingKey,
  value: string,
}

export type SettingToUpdate = Setting;

export async function readAllSettings(): Promise<Setting[]> {
  try {
    return invoke("read_all_settings");
  } catch (e) {
    // Do nothing
  }

  return [];
}

export async function readSetting(key: SettingKey): Promise<Setting | null> {
  try {
    return invoke("read_setting", { key });
  } catch (e) {
    // Do nothing
  }

  return null;
}

export async function updateSetting(arg: SettingToUpdate) {
  try {
    await invoke("update_setting", { arg: { key: arg.key, value: arg.value } });
  } catch (e) {
    // Do nothing
  }
}
