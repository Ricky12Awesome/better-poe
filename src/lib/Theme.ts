import { colors } from "./GeneratedColors";

export const defaultTheme: Theme = {
  name: "Default",
  colors: {
    foreground: colors.foreground.white,
    background: colors.background.zinc["900"],
    primary: colors.primary.violet,
    secondary: colors.secondary.purple,
    text: colors.text.white,
  },
};

export const themeToCssString = (theme: Theme) => {
  let c = theme.colors;

  return `${c.foreground} ${c.background} ${c.primary} ${c.secondary} ${c.text}`;
};

export const applyTheme = (theme: Theme) => {
  document.documentElement.className = themeToCssString(theme);
};

export type Theme = {
  name: string;
  colors: {
    foreground: string;
    background: string;
    primary: string;
    secondary: string;
    text: string;
  };
};
