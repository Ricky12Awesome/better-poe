<script lang="ts">
  import { colors } from "../lib/GeneratedColors.svelte";

  export let setColor: (newColor: string) => void;
</script>

<div class="grid grid-cols-3 text-5xl">
  {#each colors as color}
    <div class={color}>
      <button
        on:click={(event) => setColor(event.target.id)}
        id={color}
        class="hover:text-primary-50 hover:bg-primary-700 text-primary-300 bg-primary-950 px-7 py-2"
      >
        {color}
      </button>
      <br />
    </div>
  {/each}
</div>
