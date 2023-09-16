import { defaultTheme, type Theme } from "./theme";
import { colors } from "./generatedColors";

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
];

export type Settings = {
  theme: number;
};

export const defaultSettings: Settings = {
  theme: 0,
};
