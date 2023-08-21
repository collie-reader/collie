import { createSignal, For, Match, onMount, Switch } from "solid-js";

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

  const deleteFeed = async (id: number) => {
    await api.deleteFeed(id);
    setFeeds(await api.readAllFeeds());
  };

  onMount(async () => {
    setFeeds(await api.readAllFeeds());
  });

  return (
    <div class="container">
      <h2>Feeds</h2>
      <div class="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            createFeed();
          }}
        >
          <input type="text" placeholder="URL" value={linkToCreate()}
            onInput={(e) => setLinkToCreate(e.currentTarget.value)} />
          <button type="submit">Subscribe</button>
        </form>
      </div>
      <div>
        <ul>
          <For each={feeds()}>{(feed: api.Feed) =>
            <li class="row">
              <Switch>
                <Match when={feed.id === idToUpdate()}>
                  <input type="text" value={feed.title}
                    onInput={(e) => setTitleToUpdate(e.currentTarget.value)} />
                  <input type="text" value={feed.link}
                    onInput={(e) => setLinkToUpdate(e.currentTarget.value)} />
                  <button onClick={() => updateFeed(feed.id)}>Apply</button>
                </Match>
                <Match when={feed.id !== idToUpdate()}>
                  <span>{feed.title}</span>
                  <span>(<a href={feed.link} target="_blank">{feed.link}</a>)</span>
                  <button onClick={() => setIdToUpdate(feed.id)}>Edit</button>
                </Match>
              </Switch>
              <button onClick={() => deleteFeed(feed.id)}>Unsubscribe</button>
            </li>
          }</For>
        </ul>
      </div>
    </div>
  );
}

export default Feeds;
