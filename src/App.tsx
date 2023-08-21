import { lazy, createSignal } from "solid-js";
import {A, Route, Routes} from "@solidjs/router";

import "./App.css";
import { ItemStatus } from "./api/items";

const Items = lazy(() => import("./routes/Items"));
const Feeds = lazy(() => import("./routes/Feeds"));
const Settings = lazy(() => import("./routes/Settings"));

function App() {
  const [route, setRoute] = createSignal(ItemStatus.UNREAD);

  return (
    <div class="container">
      <div class="navigation row">
        <h1>Collie</h1>
        <A href="/">Unread</A>
        <A href="/saved">Saved</A>
        <A href="/read">Read</A>
        <A href="/feeds">Feeds</A>
        <A href="/settings">Settings</A>
      </div>
      <Routes>
        <Route path="/" element={<Items status={ItemStatus.UNREAD} />} />
        <Route path="/saved" element={<Items status={ItemStatus.SAVED} />} />
        <Route path="/read" element={<Items status={ItemStatus.READ} />} />
        <Route path="/feeds" component={Feeds} />
        <Route path="/settings" component={Settings} />
      </Routes>
    </div>
  );
}

export default App;
