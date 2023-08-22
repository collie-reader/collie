import { invoke } from "@tauri-apps/api/tauri";

export enum ItemStatus {
  UNREAD = "Unread",
  READ = "Read",
}

export interface Item {
  id: number,
  fingerprint: string,
  author?: string | null,
  title: string,
  description: string,
  link: string,
  status: ItemStatus,
  is_saved: boolean,
  published_at: string,
  feed: number,
}

export interface ItemToCreate {
  author?: string | null,
  title: string,
  description: string,
  link: string,
  status: ItemStatus,
  pulished_at: string,
  feed: number,
}

export interface ItemToUpdate {
  id: number,
  status?: ItemStatus | null,
  is_saved?: boolean | null,
}

export interface ItemReadOption {
  feed?: number | null,
  status?: ItemStatus | null,
  is_saved?: boolean | null,
  limit?: number | null,
  offset?: number | null,
}

export async function readItems(opt: ItemReadOption): Promise<Item[]> {
  try {
    return invoke("read_all_items", { opt: { ...opt } });
  } catch (e) {
    // Do nothing
  }

  return  [];
}

export async function save(id: number) {
  try {
    await invoke("update_item", { arg: { id, is_saved: true } });
  } catch (e) {
    // Do nothing
  }
}

export async function unsave(id: number) {
  try {
    await invoke("update_item", { arg: { id, is_saved: false } });
  } catch (e) {
    // Do nothing
  }
}

export async function markAs(id: number, status: ItemStatus) {
  try {
    await invoke("update_item", { arg: { id, status } });
  } catch (e) {
    // Do nothing
  }
}
