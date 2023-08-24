import { confirm } from '@tauri-apps/api/dialog';
import { A } from '@solidjs/router';
import { createSignal, For, Match, onMount, Switch } from "solid-js";

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import utc from 'dayjs/plugin/utc';
import timezone from 'dayjs/plugin/timezone';
dayjs.extend(relativeTime);
dayjs.extend(utc);
dayjs.extend(timezone);

import "../styles/Feeds.css";
import * as api from "../api/feeds";

function Feeds() {
  const [feeds, setFeeds] = createSignal<api.Feed[]>([]);

  const [linkToCreate, setLinkToCreate] = createSignal("");

  const [idToUpdate, setIdToUpdate] = createSignal<number | null>(null);
  const [titleToUpdate, setTitleToUpdate] = createSignal<string | null>(null);
  const [linkToUpdate, setLinkToUpdate] = createSignal<string | null>(null);

  const createFeed = async () => {
    await api.createFeed({ title: "auto", link: linkToCreate() });
    setFeeds(await api.readAllFeeds());
    setLinkToCreate("");
  };

  const updateFeed = async (id: number) => {
    await api.updateFeed({ id, title: titleToUpdate(), link: linkToUpdate() } );
    setFeeds(await api.readAllFeeds());
    setIdToUpdate(null);
    setTitleToUpdate(null);
    setLinkToUpdate(null);
  };

  const toggleFeedStatus = async (feed: api.Feed) => {
    switch (feed.status) {
      case api.FeedStatus.SUBSCRIBED:
        await api.updateFeed({ id: feed.id, status: api.FeedStatus.UNSUBSCRIBED } );
        break;
      case api.FeedStatus.UNSUBSCRIBED:
        await api.updateFeed({ id: feed.id, status: api.FeedStatus.SUBSCRIBED } );
    }

    setFeeds(await api.readAllFeeds());
  };

  const deleteFeed = async (feed: api.Feed) => {
    if (await confirm(`A feed "${feed.title}" and their all items will be deleted. Are you sure?`)) {
      await api.deleteFeed(feed.id);
      setFeeds(await api.readAllFeeds());
    }
  };

  onMount(async () => {
    setFeeds(await api.readAllFeeds());
  });

  return (
    <div class="feeds-page container">
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
      </form>
      <ul class="feed-list">
        <For each={feeds()}>{(feed: api.Feed) =>
          <li class={`${feed.status == api.FeedStatus.UNSUBSCRIBED ? "lowp" : ""}`}>
            <div class="row">
              <Switch>
                <Match when={feed.id !== idToUpdate()}>
                  <strong><A href={`/feeds/${feed.id}`}>{feed.title}</A></strong>
                  <button onClick={() => setIdToUpdate(feed.id)}>Edit</button>
                </Match>
                <Match when={feed.id === idToUpdate()}>
                  <input type="text" value={feed.title}
                    onInput={(e) => setTitleToUpdate(e.currentTarget.value)} />
                  <input type="text" value={feed.link}
                    onInput={(e) => setLinkToUpdate(e.currentTarget.value)} />
                  <button onClick={() => updateFeed(feed.id)}>Apply</button>
                </Match>
              </Switch>
            </div>
            <small>
              <span class="sep">Last checked at </span>
              <span title={dayjs(feed.checked_at).tz(dayjs.tz.guess()).format()}>
                {dayjs(feed.checked_at).fromNow()}</span>
              <span class="sep"> | </span> <a href={feed.link} target="_blank">Raw</a>
              <span class="sep"> | </span>
              <Switch>
                <Match when={feed.status === api.FeedStatus.SUBSCRIBED}>
                  <button onClick={() => toggleFeedStatus(feed)}>Unsubscribe</button>
                </Match>
                <Match when={feed.status === api.FeedStatus.UNSUBSCRIBED}>
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
  );
}

export default Feeds;
