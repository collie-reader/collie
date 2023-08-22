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
  const [opt] = createSignal<api.ItemReadOption>(option());

  const [items, setItems] = createSignal<api.Item[]>([]);
  const [selectedItem, setSelectedItem] = createSignal<api.Item | null>(null);

  const loadNext = async () => {
    setOffset(offset() + 1);
    window.scroll(0, 0);
    setItems(await api.readItems({ ...opt(), offset: offset() }));
  };

  const loadFirst = async () => {
    setOffset(0);
    window.scroll(0, 0);
    setItems(await api.readItems({ ...opt(), offset: offset() }));
  };

  const loadPrev = async () => {
    setOffset(offset() - 1);
    window.scroll(0, 0);
    setItems(await api.readItems({ ...opt(), offset: offset() }));
  };

  const toggleSave = async (item: api.Item) => {
    if (item.is_saved) {
      api.unsave(item.id)
    } else{
      api.save(item.id)
    }

    setItems(await api.readItems({ ...opt(), offset: offset() }));
  }

  const markAs = async (id: number, status: api.ItemStatus) => {
    if (status !== opt().status) {
      await api.markAs(id, status);
      setItems(await api.readItems(opt()));
    }
  };

  onMount(async () => {
    listen('feed_updated', async (_) => {
      setItems(await api.readItems(opt()));
    });

    setItems(await api.readItems(opt()));
  });

  return (
    <div class="container">
      <div class="row">
        <div class="item-list">
          <h2>{props.type.valueOf()}</h2>
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
              <button onClick={() => loadFirst()}>←← 1</button>
            </Show>
            <Show when={offset() > 0}>
              <button onClick={() => loadPrev()}>← {offset()}</button>
            </Show>
            <Show when={items().length >= LIMIT}>
              <button onClick={() => loadNext()}>{offset() + 2} →</button>
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
