<script lang="ts">
  import { writable } from "svelte/store";
  import { applyTheme } from "../../lib/theme";
  import { getSettings, themes } from "../../lib/settings";

  let settings = getSettings();
  let themeIndex = writable(0);

  settings.update((settings) => {
    let index = themes.findIndex((theme) => theme.name === settings.theme);

    if (index == -1) {
      index = 0;
    }

    themeIndex = writable(index);
    return settings;
  });

  themeIndex.subscribe((themeIndex) => {
    settings.update((settings) => {
      settings.theme = themes[themeIndex].name;
      return settings;
    });

    applyTheme(themes[themeIndex]);
  });
</script>

{#each themes as theme, i}
  <button
    id={i.toString()}
    on:click={(event) =>
      themeIndex.set(Number.parseInt(event.currentTarget.id))}
    class="hover:text-text2 px-3 py-1 text-3xl text-text hover:bg-primary"
  >
    {theme.name}
  </button>
{/each}
