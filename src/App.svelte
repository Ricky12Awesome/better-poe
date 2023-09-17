<script lang="ts">
  import Settings from "./views/Settings.svelte";
  import Stashes from "./views/Stashes.svelte";
  import Home from "./views/Home.svelte";
  import { setSettingsContext } from "./lib/settings.js";
  import { setStateContext, state } from "./lib/state";

  const views = [
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

  let id = $state?.last_page ?? 0;

  setStateContext();
  setSettingsContext();

  function select(event: any) {
    id = Number.parseInt(event.target.id);

    if ($state) $state.last_page = id;
  }
</script>

<main>
  <div class="flex flex-grow">
    <ul class="flex flex-grow">
      {#each views as option, i}
        <li>
          {#if id === i}
            <button
              id={i.toString()}
              on:click={select}
              class="text-text2 bg-primary px-8 py-3 text-center text-3xl"
            >
              {option.name}
            </button>
          {:else}
            <button
              id={i.toString()}
              on:click={select}
              class="hover:text-text2 px-8 py-3 text-center text-3xl text-text hover:bg-secondary"
            >
              {option.name}
            </button>
          {/if}
        </li>
      {/each}
    </ul>
  </div>

  <div class="bg-primary py-0.5"></div>
  <svelte:component this={views[id].component} />
</main>

<style>
</style>
