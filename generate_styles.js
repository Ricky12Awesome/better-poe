import Colors from "tailwindcss/colors";
import fs from "fs";

const blacklist = [
  // Deprecated
  "lightBlue",
  "warmGray",
  "trueGray",
  "coolGray",
  "blueGray",
  // Exclude
  "inherit",
  "current",
  "transparent",
  "black",
  "white",
];

const colors = ["primary", "secondary", "accent", "text"];

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

let svelte = '<script lang="ts" context="module">\n';
let css = "";

svelte += "  export const colors = [\n";

for (const colorName in Colors) {
  if (blacklist.find((elm) => elm === colorName)) {
    continue;
  }

  let colorVariants = Colors[colorName];

  css += `.${colorName} {\n`;
  svelte += `    "${colorName}",\n`;

  colors.forEach((name) => {
    for (const variant in colorVariants) {
      let value = colorVariants[variant];
      let rgb = hexToRgb(value);

      css += `  --${name}-${variant}: ${rgb};\n`;
    }
  });

  css += "}\n";
}

svelte += "  ];\n";
svelte += "</script>";

fs.writeFileSync("./src/styles/generatedColors.css", css);
fs.writeFileSync("./src/lib/GeneratedColors.svelte", svelte);
