<script lang="ts">
  import { type Theme, applyTheme } from "../lib/theme";
  import { getContext } from "svelte";

  const themes: Theme[] = getContext("themes");

  const setTheme = (index: number) => {
    applyTheme(themes[index]);
  };

  let options = ["API", "Theme", "Idk", "Settings"];
  let selected = 0;
</script>

<div class="flex flex-grow">
  <div>
    <ul>
      {#each options as name, i}
        <li class="flex flex-grow">
          <button
            id={i.toString()}
            on:click={(event) =>
              (selected = Number.parseInt(event.currentTarget.id))}
            class="hover:bg-primary text-text flex flex-grow px-7 py-2 text-3xl"
          >
            {name}
          </button>
        </li>
      {/each}
    </ul>
  </div>
  <div class="px-7 py-2">
    <pre class="text-7xl">{options[selected]}</pre>
  </div>
</div>
