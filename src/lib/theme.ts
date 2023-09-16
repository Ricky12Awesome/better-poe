import { colors } from "./generatedColors";

export const defaultTheme: Theme = {
  name: "Dark",
  colors: {
    foreground: colors.foreground.white,
    background: colors.background.zinc["900"],
    primary: colors.primary.violet["800"],
    secondary: colors.secondary.purple["700"],
    text: colors.text.white,
    text2: colors.text2.white,
  },
};

export const themeToCssString = (theme: Theme) => {
  let c = theme.colors;

  return `${c.foreground} ${c.background} ${c.primary} ${c.secondary} ${c.text} ${c.text2}`;
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
    text2: string;
  };
};
