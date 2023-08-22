import { invoke } from "@tauri-apps/api/tauri";

export enum FeedStatus {
  SUBSCRIBED = "Subscribed",
  UNSUBSCRIBED = "Unsubscribed",
}

export interface Feed {
    id: number,
    title: string,
    link: string,
    status: FeedStatus,
    checked_at: string,
}

export interface FeedToCreate {
    title: string,
    link: string,
}

export interface FeedToUpdate {
    id: number,
    title?: string | null,
    link?: string | null,
    status?: FeedStatus | null,
}

export async function createFeed(arg: FeedToCreate) {
  try {
    await invoke("create_feed", { arg });
  } catch (e) {
    // Do nothing
  }
}

export async function updateFeed(arg: FeedToUpdate) {
  try {
    await invoke("update_feed", { arg });
  } catch (e) {
    // Do nothing
  }
}

export async function readAllFeeds(): Promise<Feed[]> {
  try {
    return invoke("read_all_feeds");
  } catch (e) {
    // Do nothing
  }

  return  [];
}

export async function deleteFeed(id: number) {
  try {
    await invoke("delete_feed", { id });
  } catch (e) {
    // Do nothing
  }
}
