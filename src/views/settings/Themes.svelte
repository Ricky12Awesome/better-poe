<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import { applyTheme } from "../../lib/theme";
  import { type Settings, themes } from "../../lib/settings";
  import { writable } from "svelte/store";

  let settings: Writable<Settings> = getContext("Settings");
  let themeIndex = writable(0);

  settings.update((settings) => {
    themeIndex = writable(settings.theme);
    return settings;
  });

  themeIndex.subscribe((themeIndex) => {
    settings.update((settings) => {
      settings.theme = themeIndex;
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
