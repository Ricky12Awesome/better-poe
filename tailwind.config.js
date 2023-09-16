/** @type {import("tailwindcss").Config} */
export default {
  darkMode: "media",
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      //rgb(var(--color-primary) / <alpha-value>)
      colors: {
        primary: "rgb(var(--primary) / <alpha-value>)",
        secondary: "rgb(var(--secondary) / <alpha-value>)",
        background: "rgb(var(--background) / <alpha-value>)",
        foreground: "rgb(var(--foreground) / <alpha-value>)",
        text: "rgb(var(--text) / <alpha-value>)",
        text2: "rgb(var(--text2) / <alpha-value>)",
      },
    },
  },
  plugins: [],
};
