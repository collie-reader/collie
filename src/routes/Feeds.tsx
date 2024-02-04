import { confirm } from '@tauri-apps/api/dialog';
import { A, useNavigate } from '@solidjs/router';
import { createSignal, For, Match, onMount, Switch } from "solid-js";

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import utc from 'dayjs/plugin/utc';
import timezone from 'dayjs/plugin/timezone';
dayjs.extend(relativeTime);
dayjs.extend(utc);
dayjs.extend(timezone);

import "../styles/Feeds.css";
import * as feedApi from "../api/feeds";
import * as settingApi from "../api/settings";

function Feeds() {
  const navigate = useNavigate();

  const [feeds, setFeeds] = createSignal<feedApi.Feed[]>([]);
  const [linkToCreate, setLinkToCreate] = createSignal("");
  const [idToUpdate, setIdToUpdate] = createSignal<number | null>(null);
  const [titleToUpdate, setTitleToUpdate] = createSignal<string | null>(null);
  const [linkToUpdate, setLinkToUpdate] = createSignal<string | null>(null);
  const [fetchOldItems, setFetchOldItems] = createSignal<boolean>(true);

  const createFeed = async () => {
    await feedApi.createFeed({ title: "auto", link: linkToCreate(), fetch_old_items: fetchOldItems() });
    setFeeds(await feedApi.readAllFeeds());
    setLinkToCreate("");
  };

  const updateFeed = async (id: number) => {
    await feedApi.updateFeed({ id, title: titleToUpdate(), link: linkToUpdate() } );
    setFeeds(await feedApi.readAllFeeds());
    setIdToUpdate(null);
    setTitleToUpdate(null);
    setLinkToUpdate(null);
  };

  const toggleFeedStatus = async (feed: feedApi.Feed) => {
    switch (feed.status) {
      case feedApi.FeedStatus.SUBSCRIBED:
        await feedApi.updateFeed({ id: feed.id, status: feedApi.FeedStatus.UNSUBSCRIBED } );
        break;
      case feedApi.FeedStatus.UNSUBSCRIBED:
        await feedApi.updateFeed({ id: feed.id, status: feedApi.FeedStatus.SUBSCRIBED } );
    }

    setFeeds(await feedApi.readAllFeeds());
  };

  const deleteFeed = async (feed: feedApi.Feed) => {
    if (await confirm(`A feed "${feed.title}" and their all items will be deleted. Are you sure?`)) {
      await feedApi.deleteFeed(feed.id);
      setFeeds(await feedApi.readAllFeeds());
    }
  };

  const enableFetchOldItems = async (value: boolean) => {
    const real_value = value === true ? '1' : '0';
    await settingApi.updateSetting({ key: settingApi.SettingKey.FETCH_OLD_ITEMS, value: real_value });
    setFetchOldItems(value);
  }

  onMount(async () => {
    const res = await settingApi.readSetting(settingApi.SettingKey.FETCH_OLD_ITEMS);
    const value = res?.value;
    setFetchOldItems(value === '1' ? true : false);

    setFeeds(await feedApi.readAllFeeds());
  });

  return (
    <div class="feeds-page container">
      <div class="scrollable">
      <h2>Feeds</h2>
      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          createFeed();
        }}
      >
        <input type="text" placeholder="URL" value={linkToCreate()}
          onInput={(e) => setLinkToCreate(e.currentTarget.value)} />
        <button type="submit">Add & Subscribe</button>
        <span class="row">
          <input type="checkbox" id="fetch_old_items" name="fetch_old_items" checked={fetchOldItems()} onChange={(e) => enableFetchOldItems(e.currentTarget.checked)} />
          <label for="fetch_old_items"><small>Fetch old items</small></label>
        </span>
      </form>
      <ul class="feed-list">
        <For each={feeds()}>{(feed: feedApi.Feed) =>
          <li class={`${feed.status == feedApi.FeedStatus.UNSUBSCRIBED ? "lowp" : ""}`}>
            <div class="row">
              <Switch>
                <Match when={feed.id !== idToUpdate()}>
                  <strong><A href={feed.link} target="_blank">{feed.title}</A></strong>
                </Match>
                <Match when={feed.id === idToUpdate()}>
                  <input type="text" value={feed.title}
                    onInput={(e) => setTitleToUpdate(e.currentTarget.value)} />
                  <input type="text" value={feed.link}
                    onInput={(e) => setLinkToUpdate(e.currentTarget.value)} />
                </Match>
              </Switch>
              <button onClick={() => navigate(`/feeds/${feed.id}`)}>Items</button>
            </div>
            <small>
              <span class="sep">Last checked at </span>
              <span title={dayjs(feed.checked_at).tz(dayjs.tz.guess()).format()}>
                {dayjs(feed.checked_at).fromNow()}</span>
              <span class="sep"> | </span>
              <Switch>
                <Match when={feed.id !== idToUpdate()}>
                  <button onClick={() => setIdToUpdate(feed.id)}>Edit</button>
                </Match>
                <Match when={feed.id === idToUpdate()}>
                  <button onClick={() => updateFeed(feed.id)}>Apply</button>
                </Match>
              </Switch>
              <span class="sep"> | </span>
              <Switch>
                <Match when={feed.status === feedApi.FeedStatus.SUBSCRIBED}>
                  <button onClick={() => toggleFeedStatus(feed)}>Unsubscribe</button>
                </Match>
                <Match when={feed.status === feedApi.FeedStatus.UNSUBSCRIBED}>
                  <button onClick={() => toggleFeedStatus(feed)}>Subscribe</button>
                </Match>
              </Switch>
              <span class="sep"> | </span>
              <button onClick={() => deleteFeed(feed)}>Delete</button>
            </small>
          </li>
        }</For>
      </ul>
      </div>
    </div>
  );
}

export default Feeds;
