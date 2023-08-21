/* @refresh reload */
import { render } from "solid-js/web";
import { Router } from "@solidjs/router";

import "./styles.css";
import App from "./App";

render(() => <Router><App /></Router>, document.getElementById("root") as HTMLElement);
