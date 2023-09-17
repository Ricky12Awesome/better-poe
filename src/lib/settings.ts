import { applyTheme, defaultTheme, type Theme } from "./theme";
import { colors } from "./gen/colors";

import { type Settings } from "./gen/types";
import { type Writable, writable } from "svelte/store";
import { getContext, setContext } from "svelte";
import { invoke } from "@tauri-apps/api";

export const settings: Writable<Settings | undefined> = writable(undefined);

export const SETTINGS_CONTEXT_KEY = "Settings";

export const themes: Theme[] = [
  defaultTheme,
  {
    name: "Light",
    colors: {
      foreground: colors.foreground.black,
      background: colors.background.zinc["50"],
      primary: colors.primary.violet["600"],
      secondary: colors.secondary.purple["400"],
      text: colors.text.black,
      text2: colors.text2.white,
    },
  },
  {
    name: "Red",
    colors: {
      foreground: colors.foreground.black,
      background: colors.background.zinc["50"],
      primary: colors.primary.red["600"],
      secondary: colors.secondary.red["400"],
      text: colors.text.black,
      text2: colors.text2.white,
    },
  },
];

export const getSettings = (): Writable<Settings> => {
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

  return settings;
};

export { type Settings };
