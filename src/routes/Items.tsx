import { createSignal, For, Match, onMount, Show, Switch } from "solid-js";

import * as api from "../api/items";

interface Props {
  status: api.ItemStatus;
}

function Items(props: Props) {
  const [offset, setOffset] = createSignal(0);
  const [opt, setOpt] = createSignal<api.ItemReadOption>({ status: props.status, offset: offset(), limit: 50 });

  const [items, setItems] = createSignal<api.Item[]>([]);
  const [selectedItem, setSelectedItem] = createSignal<api.Item | null>(null);

  const loadNext = async () => {
    setOffset(offset() + 1);
    setItems(await api.readItems({ ...opt(), offset: offset() }));
  };

  const loadPrev = async () => {
    setOffset(offset() - 1);
    setItems(await api.readItems({ ...opt(), offset: offset() }));
  };

  const markAs = async (id: number, status: api.ItemStatus) => {
    await api.markAs(id, status);
    setItems(await api.readItems(opt()));
  };

  onMount(async () => {
    setItems(await api.readItems(opt()));
  });

  return (
    <div class="container">
      <div class="row">
        <div class="item-list">
          <h2>{props.status.valueOf()}</h2>
          <div>
            <ul>
              <For each={items()}>{(item: api.Item) =>
                <li>
                  <a href={item.link} target="_blank">{item.title}</a>
                  <span>by {item.author}</span>
                  <span>on {item.feed}</span>
                  <span>at {item.published_at}</span>
                  <a onClick={() => markAs(item.id, api.ItemStatus.SAVED)}>Save</a>
                  <a onClick={() => setSelectedItem(item)}>More</a>
                </li>
              }</For>
            </ul>
          </div>
          <div class="row">
            <Show when={offset() > 0}>
              <button onClick={() => loadPrev()}>←</button>
            </Show>
            <span>{offset()}</span>
            <button onClick={() => loadNext()}>→</button>
          </div>
        </div>
        <Show when={selectedItem()}>
          <div class="item-viewer">
            <div>
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
