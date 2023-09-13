import "./styles.css";
import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app"),
} as any); // as any because ide gives false-positive errors

export default app;
