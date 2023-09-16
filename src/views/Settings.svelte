<script lang="ts">
  import API from "./settings/API.svelte";
  import WIP from "../lib/WIP.svelte";
  import Themes from "./settings/Themes.svelte";

  const options = [
    {
      name: "API",
      component: API,
    },
    {
      name: "Themes",
      component: Themes,
    },
    {
      name: "Option 3",
      component: WIP,
    },
    {
      name: "Option 4",
      component: WIP,
    },
    {
      name: "Option 5",
      component: WIP,
    },
    {
      name: "Option 6",
      component: WIP,
    },
    {
      name: "Option 7",
      component: WIP,
    },
  ];

  let selected = 0;
</script>

<div class="flex flex-grow">
  <div>
    <ul>
      {#each options as { name }, i}
        <li class="flex flex-grow">
          {#if selected === i}
            <button
              id={i.toString()}
              on:click={(event) =>
                (selected = Number.parseInt(event.currentTarget.id))}
              class="text-text2 flex flex-grow bg-primary px-7 py-2 text-3xl hover:bg-primary"
            >
              {name}
            </button>
          {:else}
            <button
              id={i.toString()}
              on:click={(event) =>
                (selected = Number.parseInt(event.currentTarget.id))}
              class="hover:text-text2 flex flex-grow px-7 py-2 text-3xl text-text hover:bg-primary"
            >
              {name}
            </button>
          {/if}
        </li>
      {/each}
    </ul>
  </div>
  <!--  <div class="bg-primary px-0.5"></div>-->
  <div class="px-7 py-2">
    <svelte:component this={options[selected].component} />
  </div>
</div>
