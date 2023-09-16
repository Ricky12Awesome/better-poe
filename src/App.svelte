<script lang="ts">
  import Settings from "./views/Settings.svelte";
  import Stashes from "./views/Stashes.svelte";
  import { colors } from "./lib/generatedColors";
  import { applyTheme, defaultTheme, type Theme } from "./lib/theme";
  import { setContext } from "svelte";

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
        primary: colors.primary.blue["700"],
        secondary: colors.secondary.blue["600"],
        text: colors.text.blue["50"],
      },
    },
    {
      name: "Red",
      colors: {
        foreground: colors.foreground.neutral[50],
        background: colors.background.red["950"],
        primary: colors.primary.red["700"],
        secondary: colors.secondary.red["600"],
        text: colors.text.red["50"],
      },
    },
    {
      name: "Light",
      colors: {
        foreground: colors.foreground.black,
        background: colors.background.white,
        primary: colors.primary.violet["500"],
        secondary: colors.secondary.violet["100"],
        text: colors.text.black,
      },
    },
  ];

  let selectedView = options[0];
  let selectedViewId = 0;

  function select(event: any) {
    selectedView = options[event.target.id];
    selectedViewId = Number.parseInt(event.target.id);
  }

  setContext("themes", themes);
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
              class="bg-primary text-text hover:bg-primary hover:text-text px-8 py-3 text-center text-3xl"
            >
              {option.name}
            </button>
          {:else}
            <button
              id={i.toString()}
              on:click={select}
              class="text-text hover:bg-primary hover:text-text px-8 py-3 text-center text-3xl"
            >
              {option.name}
            </button>
          {/if}
        </li>
      {/each}
    </ul>
  </div>

  <div class="bg-primary py-0.5"></div>
  <svelte:component this={selectedView.component} />
</main>

<style>
</style>
