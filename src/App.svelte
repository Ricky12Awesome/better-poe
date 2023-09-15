<script lang="ts">
  import Settings from "./views/Settings.svelte";
  import Stashes from "./views/Stashes.svelte";
  import { colors } from "./lib/GeneratedColors";
  import { applyTheme, defaultTheme, type Theme } from "./lib/Theme";

  applyTheme(defaultTheme);

  const options = [
    {
      name: "Settings",
      component: Settings,
    },
    {
      name: "Stashes",
      component: Stashes,
    },
  ];
  const themes: Theme[] = [
    defaultTheme,
    {
      name: "Blue",
      colors: {
        foreground: colors.foreground.neutral[50],
        background: colors.background.slate["900"],
        primary: colors.primary.blue,
        secondary: colors.secondary.blue,
        text: colors.text.blue,
      },
    },
    {
      name: "Red",
      colors: {
        foreground: colors.foreground.neutral[50],
        background: colors.background.red["950"],
        primary: colors.primary.red,
        secondary: colors.secondary.red,
        text: colors.text.red,
      },
    },
  ];

  let selectedView = options[0];
  let selectedViewId = 0;

  function select(event: any) {
    selectedView = options[event.target.id];
    selectedViewId = Number.parseInt(event.target.id);
  }
</script>

<main>
  <div class="flex flex-grow">
    <ul class="flex flex-grow">
      {#each options as option, i}
        <li>
          {#if selectedViewId === i}
            <button
              id={i.toString()}
              on:click={select}
              class="bg-primary-700 px-8 py-3 text-center text-4xl text-text-100 hover:bg-primary-700 hover:text-text-200"
            >
              {option.name}
            </button>
          {:else}
            <button
              id={i.toString()}
              on:click={select}
              class="px-8 py-3 text-center text-4xl text-text-100 hover:bg-primary-700 hover:text-text-200"
            >
              {option.name}
            </button>
          {/if}
        </li>
      {/each}
    </ul>
  </div>

  <svelte:component this={selectedView.component} {themes} />
</main>

<style>
</style>
