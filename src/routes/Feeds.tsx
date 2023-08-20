import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

function Feeds() {
  const [link, setLink] = createSignal("");

  const create_feed = async () => {
    const a: string = await invoke("create_feed", { link: link() });
    setLink(a);
  };

  return (
    <div class="container">
      <h2>Feeds</h2>
      <div class="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            create_feed();
          }}
        >
          <input type="text" placeholder="URL" onChange={(e) => setLink(e.currentTarget.value)} />
          <button type="submit">Add</button>
        </form>
      </div>
    </div>
  );
}

export default Feeds;
