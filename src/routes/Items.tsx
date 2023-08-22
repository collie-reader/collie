import { listen } from '@tauri-apps/api/event';
import { createSignal, For, Match, onMount, Show, Switch } from "solid-js";

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import utc from 'dayjs/plugin/utc';
import timezone from 'dayjs/plugin/timezone';
dayjs.extend(relativeTime);
dayjs.extend(utc);
dayjs.extend(timezone);

import "../styles/Items.css";
import * as api from "../api/items";

export enum ItemType {
  INBOX = "Inbox",
  UNREAD = "Unread",
  SAVED = "Saved",
}

interface Props {
  type: ItemType;
}

function Items(props: Props) {
  const LIMIT = 50;

  function option(): api.ItemReadOption {
    let opt = { offset: offset(), limit: LIMIT };

    switch (props.type) {
      case ItemType.INBOX:
        return opt;
      case ItemType.UNREAD:
        return { ...opt, status: api.ItemStatus.UNREAD };
      case ItemType.SAVED:
        return { ...opt, is_saved: true };
    }
  }

  const [offset, setOffset] = createSignal(0);
  const [opt, setOpt] = createSignal<api.ItemReadOption>(option());

  const [items, setItems] = createSignal<api.Item[]>([]);
  const [selectedItem, setSelectedItem] = createSignal<api.Item | null>(null);
  const [count, setCount] = createSignal(0);

  const loadItems = async () => {
    setCount(await api.countItems(opt()));
    setItems(await api.readItems(opt()));
  };

  const loadPage = async (newOffset: number) => {
    setOffset(newOffset);
    setOpt({ ...opt(), offset: offset() })
    window.scroll(0, 0);
    await loadItems();
  };

  const toggleSave = async (item: api.Item) => {
    if (item.is_saved) {
      await api.unsave(item.id)
    } else{
      await api.save(item.id)
    }

    await loadItems();
  }

  const markAs = async (id: number, status: api.ItemStatus) => {
    if (status !== opt().status) {
      await api.markAs(id, status);
      await loadItems();
    }
  };

  onMount(async () => {
    listen('feed_updated', async (_) => {
      await loadItems();
    });

    await loadItems();
  });

  return (
    <div class="container">
      <div class="row">
        <div class="item-list">
          <h2>{props.type.valueOf()} ({count()})</h2>
          <ul>
            <For each={items()}>{(item: api.Item) =>
              <li class={`${item.status == api.ItemStatus.READ ? "lowp" : ""}`}>
                <strong><a href={item.link} target="_blank"
                  onClick={() => markAs(item.id, api.ItemStatus.READ)}>{item.title}</a></strong>
                <small class="row">
                  <span class="sep">on </span>{item.feed.title}
                  <span class="sep"> by </span>{item.author}
                  <span class="sep"> at </span><span title={dayjs(item.published_at).tz(dayjs.tz.guess()).format()}>{dayjs(item.published_at).fromNow()}</span>
                  <span class="sep"> | </span>
                  <Switch>
                    <Match when={!item.is_saved}><button onClick={() => toggleSave(item)}>Save</button></Match>
                    <Match when={item.is_saved}><button onClick={() => toggleSave(item)}>Unsave</button></Match>
                  </Switch>
                  <span class="sep"> | </span>
                  <button onClick={() => {
                    setSelectedItem(item);
                    markAs(item.id, api.ItemStatus.READ)
                  }}>More</button>
                </small>
              </li>
            }</For>
          </ul>
          <div class="row">
            <Show when={offset() > 3}>
              <button onClick={() => loadPage(0)}>←← 1</button>
            </Show>
            <Show when={offset() > 0}>
              <button onClick={() => loadPage(offset() - 1)}>← {offset()}</button>
            </Show>
            <Show when={(offset() + 1) * LIMIT < count()}>
              <button onClick={() => loadPage(offset() + 1)}>{offset() + 2} →</button>
            </Show>
          </div>
        </div>
        <Show when={selectedItem()}>
          <div class="item-viewer-container">
            <div class="item-viewer">
              <h3>{selectedItem()?.title}</h3>
              <div innerHTML={selectedItem()?.description} />
            </div>
          </div>
        </Show>
      </div>
    </div>
  );
}

export default Items;
