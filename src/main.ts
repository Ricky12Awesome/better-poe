import "./styles.css";
import App from "./App.svelte";
import { loadSettings } from "./lib/settings";
import { loadState } from "./lib/state";

await loadState();
await loadSettings();

const app = new App({
  target: document.getElementById("app"),
} as any); // as any because ide gives false-positive errors

export default app;
