import "./styles.css";
import App from "./App.svelte";
import { loadSettings, loadState, loadToken } from "./lib/storage";

await loadState();
await loadSettings();
await loadToken();

const app = new App({
  target: document.getElementById("app"),
} as any); // as any because ide gives false-positive errors

export default app;
