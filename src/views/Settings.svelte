<script lang="ts">
  import { type Theme, themeToCssString, applyTheme } from "../lib/Theme";

  export let themes: Theme[];

  const setTheme = (index: number) => {
    applyTheme(themes[index]);
  };
</script>

<div class="grid grid-cols-2 text-5xl">
  {#each themes as theme, i}
    <div class={themeToCssString(theme)}>
      <button
        on:click={(event) => setTheme(Number.parseInt(event.currentTarget.id))}
        id={i.toString()}
        class="bg-primary-950 px-7 py-2 text-text-300 hover:bg-primary-600 hover:text-secondary-50"
      >
        {theme.name}
      </button>
      <br />
    </div>
  {/each}
</div>
