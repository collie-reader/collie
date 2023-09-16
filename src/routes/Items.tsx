import { listen } from '@tauri-apps/api/event';
import { A, useParams } from '@solidjs/router';
import { createSignal, For, Match, onMount, Show, Switch } from "solid-js";
import DOMPurify from 'dompurify';

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import utc from 'dayjs/plugin/utc';
import timezone from 'dayjs/plugin/timezone';
dayjs.extend(relativeTime);
dayjs.extend(utc);
dayjs.extend(timezone);

import "../styles/Items.css";
import * as api from "../api/items";
import * as feedApi from "../api/feeds";
import * as settingApi from "../api/settings";
import { ItemType } from './models/items';

interface Props {
  type: ItemType;
}

function Items(props: Props) {
  const LIMIT = 50;
  const params = useParams();

  const [offset, setOffset] = createSignal(0);
  const [opt, setOpt] = createSignal<api.ItemReadOption>({});
  const [feed, setFeed] = createSignal<feedApi.Feed | null>(null);
  const [items, setItems] = createSignal<api.Item[]>([]);
  const [selectedItem, setSelectedItem] = createSignal<api.Item | null>(null);
  const [count, setCount] = createSignal(0);
  const [viewerBasis, setViewerBasis] = createSignal(200);

  const loadItems = async () => {
    const [fetchedCount, fetchedItems] = await Promise.all([
      api.countItems(opt()),
      api.readItems(opt()),
    ]);

    setCount(fetchedCount);
    setItems(fetchedItems);
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

  const markAs = async (targets: api.Item[], status: api.ItemStatus) => {
    const ids = targets.filter(x => x.status !== status).map(x => x.id);
    if (ids.length) {
      await api.markAs(ids, status);
      await loadItems();
    }
  };

  const changeOrder = async (order: api.ItemOrder) => {
    setOpt({ ...opt(), order_by: order });
    await Promise.all([
      settingApi.updateSetting({ key: settingApi.SettingKey.ITEMS_ORDER, value: order }),
      loadItems(),
    ]);
  }

  const selectItem = (item: api.Item) => {
    setSelectedItem(item);
    markAs([item], api.ItemStatus.READ)

    const resize = (e: MouseEvent) => {
      e.preventDefault();
      const basis = document.documentElement.clientWidth - e.clientX - 60;
      if (basis >= 100 && basis < 800) {
        setViewerBasis(basis);
      }
    };

    document.querySelector(".item-viewer-handle")?.addEventListener("mousedown", () => {
      document.addEventListener("mousemove", resize, false);
      document.addEventListener("mouseup", () => {
        document.removeEventListener("mousemove", resize, false);
      }, false);
    });
  };

  onMount(async () => {
    const res = await settingApi.readSetting(settingApi.SettingKey.ITEMS_ORDER);
    const order = res?.value ?? api.ItemOrder.RECEIVED_DATE_DESC;

    let initialOpt = { order_by: api.ItemOrderfrom(order), offset: 0, limit: LIMIT };
    switch (props.type) {
      case ItemType.INBOX:
        setOpt(initialOpt);
        break;
      case ItemType.UNREAD:
        setOpt({ ...initialOpt, status: api.ItemStatus.UNREAD });
        break;
      case ItemType.SAVED:
        setOpt({ ...initialOpt, is_saved: true });
        break;
      case ItemType.FEED:
        setOpt({ ...initialOpt, feed: Number(params.id) });
    }

    if (props.type === ItemType.FEED) {
      const [fetchedFeed] = await Promise.all([
        feedApi.readFeed(Number(params.id)),
        loadItems(),
      ]);

      setFeed(fetchedFeed);
    } else {
      await loadItems();
    }
  });

  // eslint-disable-next-line solid/reactivity
  listen('feed_updated', async () => loadItems());

  return (
    <div class="items-page container row">
      <div class="item-list scrollable">
        <Switch fallback={<h2>{`${props.type.valueOf()} (${count()})`}</h2>}>
          <Match when={props.type == ItemType.FEED}>
            <h2>
              <a onClick={() => history.back()}>←</a>
              <span> {`${feed() ? feed()?.title : 'Feed'} (${count()})`}</span>
            </h2>
          </Match>
        </Switch>
        <div class="row controls-container">
          <label for="sort">Sort by</label>
          <select name="sort" onChange={(e) => changeOrder(api.ItemOrderfrom(e.target.value))}>
            <option selected={opt().order_by === api.ItemOrder.RECEIVED_DATE_DESC}
              value={api.ItemOrder.RECEIVED_DATE_DESC}>Received date</option>
            <option selected={opt().order_by === api.ItemOrder.PUBLISHED_DATE_DESC}
              value={api.ItemOrder.PUBLISHED_DATE_DESC}>Published date</option>
            <option selected={opt().order_by === api.ItemOrder.UNREAD_FIRST}
              value={api.ItemOrder.UNREAD_FIRST}>Unread first</option>
          </select>
          <Show when={items().length && items().some(x => x.status == api.ItemStatus.UNREAD)}>
            <button onClick={() => markAs(items(), api.ItemStatus.READ)}>
              Mark this page as read</button>
          </Show>
        </div>
        <ul>
          <For each={items()}>{(item: api.Item) =>
            <li class={`${item.status == api.ItemStatus.READ ? "lowp" : ""} ${(selectedItem() && selectedItem()?.id == item.id) ? "selected" : ""}`}>
              <strong><a href={item.link} target="_blank"
                onClick={() => markAs([item], api.ItemStatus.READ)}>
                {item.title} <small class="hostname">({new URL(item.link).hostname})</small>
              </a></strong>
              <small class="row">
                <span class="sep">on</span> <A href={`/feeds/${item.feed.id}`}>{item.feed.title}</A>
                <span class="sep"> by</span> {item.author}
                <span class="sep"> at </span>
                <span title={dayjs(item.published_at).tz(dayjs.tz.guess()).format()}>
                  {dayjs(item.published_at).fromNow()}</span>
                <Show when={item.status == api.ItemStatus.READ}>
                  <span class="sep"> | </span>
                  <button onClick={() => markAs([item], api.ItemStatus.UNREAD)}>Mark as unread</button>
                </Show>
                <span class="sep"> | </span>
                <Switch>
                  <Match when={!item.is_saved}><button onClick={() => toggleSave(item)}>Save</button></Match>
                  <Match when={item.is_saved}><button onClick={() => toggleSave(item)}>Unsave</button></Match>
                </Switch>
                <span class="sep"> | </span>
                <button onClick={() => selectItem(item)}>Read</button>
              </small>
            </li>
          }</For>
        </ul>
        <Show when={count() > LIMIT}>
          <div class="row">
            <Show when={offset() > 1}>
              <button onClick={() => loadPage(0)}>←← 1</button>
            </Show>
            <Show when={offset() > 0}>
              <button onClick={() => loadPage(offset() - 1)}>← {offset()}</button>
            </Show>
            <Show when={(offset() + 1) * LIMIT < count()}>
              <button onClick={() => loadPage(offset() + 1)}>{offset() + 2} →</button>
            </Show>
          </div>
        </Show>
      </div>
      <Show when={selectedItem()}>
        <div class="item-viewer-handle" />
        <div class="item-viewer scrollable" style={{ 'flex-basis': `${viewerBasis()}px` }}>
          <h2 class="heading">
            <span>{selectedItem()?.title}</span>
            <button onClick={() => setSelectedItem(null)}>✖</button>
          </h2>
          {/* eslint-disable-next-line solid/no-innerhtml*/}
          <div innerHTML={DOMPurify.sanitize(selectedItem()?.description ?? "")
            .replace(/href="http(s?).*"/g, "target=\"_blank\" $&")} />
        </div>
      </Show>
    </div>
  );
}

export default Items;
