<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  let list: string[] = ["1", "2", "3"];

  (async () => {
    await invoke("file_watcher", {
      path: "../test.txt"
    });

    await listen("test", async event => {
      list.push(event.payload as string);
      list = list;
    });
  })();
</script>

<div style="overflow-y: unset; height: 80vh">
  <code>
    {#each list as element}
      {element}<br>
    {/each}
  </code>
</div>

<style>

</style>