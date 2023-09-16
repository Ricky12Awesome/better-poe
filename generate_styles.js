import Colors from "tailwindcss/colors";
import fs from "fs";

const blacklist = [
  // Deprecated
  "lightBlue",
  "warmGray",
  "trueGray",
  "coolGray",
  "blueGray",
  // Blocked
  "inherit",
  "current",
  "transparent",
];

const colorsOne = [
  "primary",
  "secondary",
  "background",
  "foreground",
  "text",
  "text2",
];

/** @type function(string) => string */
const hexToRgb = (value) => {
  const hex = value.substring(1);
  const red = hex.substring(0, 2);
  const green = hex.substring(2, 4);
  const blue = hex.substring(4, 6);

  const r = Number.parseInt(red, 16);
  const g = Number.parseInt(green, 16);
  const b = Number.parseInt(blue, 16);

  return `${r} ${g} ${b}`;
};

let svelte = "export const colors = {\n";
let css = "";

colorsOne.forEach((name) => {
  svelte += `  ${name}: {\n`;

  for (const colorName in Colors) {
    if (blacklist.find((elm) => elm === colorName)) {
      continue;
    }

    let colorVariants = Colors[colorName];

    if (typeof colorVariants === "object") {
      svelte += `    ${colorName}: {\n`;

      for (const variant in colorVariants) {
        let value = colorVariants[variant];
        let rgb = hexToRgb(value);

        css += `.${name}-${colorName}-${variant} {\n`;
        svelte += `        ${variant}: "${name}-${colorName}-${variant}",\n`;
        css += `  --${name}: ${rgb};\n`;
        css += "}\n";
      }

      svelte += `    },\n`;
    } else if (typeof colorVariants === "string") {
      if (colorName === "white") {
        svelte += `    ${colorName}: "${name}-${colorName}",\n`;
        css += `.${name}-${colorName} {\n`;
        css += `  --${name}: 255 255 255;\n`;
        css += `}\n`;
      } else if (colorName === "black") {
        svelte += `    ${colorName}: "${name}-${colorName}",\n`;
        css += `.${name}-${colorName} {\n`;
        css += `  --${name}: 0 0 0;\n`;
        css += `}\n`;
      } else {
        let rgb = hexToRgb(colorVariants);

        svelte += `    ${colorName}: "${name}-${colorName}",\n`;
        css += `.${name}-${colorName} {\n`;
        css += `  --${name}: ${rgb};\n`;
        css += `}\n`;
      }
    }
  }

  svelte += `  },\n`;
});

svelte += "};\n";

fs.writeFileSync("./src/styles/generatedColors.css", css);
fs.writeFileSync("./src/lib/generatedColors.ts", svelte);
