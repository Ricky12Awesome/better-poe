<script lang="ts">
  import { writable } from "svelte/store";
  import { applyTheme, themes } from "../../lib/theme";
  import { settings } from "../../lib/storage";

  let index = themes.findIndex((theme) => theme.name === $settings.theme);

  if (index == -1) {
    index = 0;
  }

  let themeIndex = writable(index);

  themeIndex.subscribe((themeIndex) => {
    $settings.theme = themes[themeIndex].name;

    applyTheme(themes[themeIndex]);
  });
</script>

{#each themes as theme, i}
  <button
    id={i.toString()}
    on:click={(event) =>
      themeIndex.set(Number.parseInt(event.currentTarget.id))}
    class="px-3 py-1 text-3xl text-text hover:bg-primary hover:text-text2"
  >
    {theme.name}
  </button>
{/each}
