import { invoke } from "@tauri-apps/api/tauri";

export enum SettingKey {
  POLLING_FREQUENCY = "PollingFrequency",
  NOTIFICATION = "Notification",
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

  return  [];
}

export async function updateSetting(arg: SettingToUpdate) {
  try {
    await invoke("update_setting", { arg: { key: arg.key, value: arg.value } });
  } catch (e) {
    // Do nothing
  }
}
