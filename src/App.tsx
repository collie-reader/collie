import { lazy } from "solid-js";
import { A, Route, Routes } from "@solidjs/router";

import "./App.css";
import { ItemType } from "./routes/Items";

const Items = lazy(() => import("./routes/Items"));
const Feeds = lazy(() => import("./routes/Feeds"));
const Settings = lazy(() => import("./routes/Settings"));

function App() {
  return (
    <div class="container">
      <div class="navigation row">
        <h1>Collie</h1>
        <A href="/">Inbox</A>
        <A href="/unread">Unread</A>
        <A href="/saved">Saved</A>
        <A href="/feeds">Feeds</A>
        <A href="/settings">Settings</A>
      </div>
      <Routes>
        <Route path="/" element={<Items type={ItemType.INBOX} />} />
        <Route path="/unread" element={<Items type={ItemType.UNREAD} />} />
        <Route path="/saved" element={<Items type={ItemType.SAVED} />} />
        <Route path="/feeds" component={Feeds} />
        <Route path="/feeds/:id" element={<Items type={ItemType.FEED} />} />
        <Route path="/settings" component={Settings} />
      </Routes>
    </div>
  );
}

export default App;
