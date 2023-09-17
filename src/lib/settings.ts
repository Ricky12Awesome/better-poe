import { applyTheme, defaultTheme, themes } from "./theme";

import { type Settings } from "./gen/types";
import { type Writable, writable } from "svelte/store";
import { getContext, setContext } from "svelte";
import { invoke } from "@tauri-apps/api";

export const settings: Writable<Settings | undefined> = writable(undefined);

export const SETTINGS_CONTEXT_KEY = "Settings";

export const getSettingsContext = (): Writable<Settings> => {
  return getContext(SETTINGS_CONTEXT_KEY);
};

export const setSettingsContext = () => {
  setContext(SETTINGS_CONTEXT_KEY, settings);
};

export const loadSettings = async () => {
  let loadedSettings: Settings = await invoke("load_settings");

  settings.set(loadedSettings);

  settings.subscribe(async (settings) => {
    if (settings) await invoke("save_settings", { settings });
  });

  applyTheme(
    themes.find((theme) => theme.name === loadedSettings.theme) ?? defaultTheme,
  );
};

export { type Settings };
