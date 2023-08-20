import { createSignal, Switch, Match } from "solid-js";

import "./App.css";

import Settings from "./routes/Settings";
import List from "./routes/List";

export enum Page {
  UNREAD = "Unread",
  STARRED = "Starred",
  ARCHIVED = "Archived",
  FEEDS = "Feeds",
  SETTINGS = "Settings",
}

function App() {
  const [route, setRoute] = createSignal(Page.UNREAD);

  return (
    <div class="container">
      <div class="navigation row">
        <h1>Collie</h1>
        <a onClick={() => setRoute(Page.UNREAD)}>Unread</a>
        <a onClick={() => setRoute(Page.STARRED)}>Starred</a>
        <a onClick={() => setRoute(Page.ARCHIVED)}>Archived</a>
        <a onClick={() => setRoute(Page.FEEDS)}>Feeds</a>
        <a onClick={() => setRoute(Page.SETTINGS)}>Settings</a>
      </div>
      <Switch fallback={<List type={Page.UNREAD} />}>
        <Match when={route() == Page.UNREAD}>
          <List type={Page.UNREAD} />
        </Match>
        <Match when={route() == Page.STARRED}>
          <List type={Page.STARRED} />
        </Match>
        <Match when={route() == Page.ARCHIVED}>
          <List type={Page.ARCHIVED} />
        </Match>
        <Match when={route() == Page.FEEDS}>
          <List type={Page.FEEDS} />
        </Match>
        <Match when={route() == Page.SETTINGS}>
          <Settings />
        </Match>
      </Switch>
    </div>
  );
}

export default App;
