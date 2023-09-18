import { applyTheme, defaultTheme, themes } from "./theme";

import type { Settings, State, Token } from "./gen/types";
import { type Writable, writable } from "svelte/store";
import { invoke } from "@tauri-apps/api";

export type * from "./gen/types";
export const state: Writable<State> = writable();
export const settings: Writable<Settings> = writable();
export const token: Writable<Token | undefined> = writable();

export const loadSettings = async () => {
  let loaded: Settings = await invoke("load_settings");

  settings.set(loaded);

  settings.subscribe(async (settings) => {
    if (settings) await invoke("save_settings", { settings });
  });

  applyTheme(
    themes.find((theme) => theme.name === loaded.theme) ?? defaultTheme,
  );
};

export const loadState = async () => {
  let loaded: State = await invoke("load_state");

  state.set(loaded);

  state.subscribe(async (state) => {
    if (state)
      await invoke("save_state", {
        state,
      });
  });
};

export const loadToken = async () => {
  let loaded: Token;

  try {
    loaded = await invoke("load_token");
  } catch (_) {
    return;
  }

  token.set(loaded);
};
