<script lang="ts">
  import Settings from "./views/Settings.svelte";
  import Stashes from "./views/Stashes.svelte";

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

  let color = "purple";
  let selected = options[0];
  let selectedId = 0;

  function select(event: any) {
    selected = options[event.target.id];
    selectedId = Number.parseInt(event.target.id);
  }

  function setColor(newColor: string) {
    color = newColor;
  }
</script>

<main class={color}>
  <div class="flex flex-grow">
    <ul class="flex flex-grow">
      {#each options as option, i}
        <li>
          {#if selectedId === i}
            <button
              id={i.toString()}
              on:click={select}
              class="bg-primary-700 hover:bg-primary-700 text-text-100 hover:text-text-200 px-8 py-3 text-center text-4xl"
            >
              {option.name}
            </button>
          {:else}
            <button id={i.toString()} on:click={select} class="hover:bg-primary-700 hover:text-text-200 text-text-100 px-8 py-3 text-center text-4xl">
              {option.name}
            </button>
          {/if}
        </li>
      {/each}
    </ul>
  </div>

  <svelte:component this={selected.component} {setColor} />
</main>

<style>
</style>
