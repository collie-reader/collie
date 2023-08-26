import { invoke } from "@tauri-apps/api/tauri";

export enum ItemStatus {
  UNREAD = "Unread",
  READ = "Read",
}

export interface ItemFeed {
  id: number,
  title: string,
  link: string,
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
  feed: ItemFeed,
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

export interface ItemToUpdateAll {
  status?: ItemStatus | null,
  is_saved?: boolean | null,
  option: ItemReadOption,
}

export enum ItemOrder {
  RECEIVED_DATE_DESC = "ReceivedDateDesc",
  PUBLISHED_DATE_DESC = "PublishedDateDesc",
  UNREAD_FIRST = "UnreadFirst",
}

export function ItemOrderfrom(x: string) {
  switch (x) {
    case "ReceivedDateDesc":
      return ItemOrder.RECEIVED_DATE_DESC;
    case "PublishedDateDesc":
      return ItemOrder.PUBLISHED_DATE_DESC;
    case "UnreadFirst":
      return ItemOrder.UNREAD_FIRST;
    default:
      return ItemOrder.RECEIVED_DATE_DESC;
  }
}

export interface ItemReadOption {
  ids?: number[] | null,
  feed?: number | null,
  status?: ItemStatus | null,
  is_saved?: boolean | null,
  order_by?: ItemOrder | null,
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

export async function countItems(opt: ItemReadOption): Promise<number> {
  try {
    return invoke("count_all_items", { opt: { ...opt } });
  } catch (e) {
    // Do nothing
  }

  return  0
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

export async function markAs(ids: number[], status: ItemStatus) {
  try {
    if (ids.length === 1) {
      await invoke("update_item", { arg: { id: ids[0], status } });
    } else if (ids.length > 1) {
      await invoke("update_items", { arg: { opt: { ids }, status } });
    }
  } catch (e) {
    // Do nothing
  }
}
