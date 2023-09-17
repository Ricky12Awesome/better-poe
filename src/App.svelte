<script lang="ts">
  import Settings from "./views/Settings.svelte";
  import Stashes from "./views/Stashes.svelte";
  import Home from "./views/Home.svelte";
  import { loadSettings, setSettingsContext } from "./lib/settings.js";
  import { onMount } from "svelte";

  setSettingsContext();

  onMount(async () => {
    await loadSettings();
  });

  const options = [
    {
      name: "Home",
      component: Home,
    },
    {
      name: "Stashes",
      component: Stashes,
    },
    {
      name: "Settings",
      component: Settings,
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
              class="text-text2 bg-primary px-8 py-3 text-center text-3xl hover:bg-primary"
            >
              {option.name}
            </button>
          {:else}
            <button
              id={i.toString()}
              on:click={select}
              class="hover:text-text2 px-8 py-3 text-center text-3xl text-text hover:bg-primary"
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
