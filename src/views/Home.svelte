<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { token } from "../lib/storage";
  import { listen } from "@tauri-apps/api/event";

  let gettingToken = false;
  let error = false;

  const getToken = async () => {
    const unListen = await listen("get_token_result", async (event) => {
      await invoke("save_token", { token: event.payload });

      gettingToken = false;
      unListen();
    });

    gettingToken = true;

    invoke("get_token")
      .then(() => (gettingToken = false))
      .catch(() => {
        gettingToken = false;
        error = true;
      });
  };
</script>

<div class="pt-[45vh] text-center">
  {#if !$token && !gettingToken}
    <button
      on:click={getToken}
      class="bg-primary px-6 py-3 text-center text-4xl text-text2 hover:bg-secondary"
      >Login
    </button>
  {:else if gettingToken}
    <h1 class="text-text1 px-6 py-3 text-center text-4xl">Logging in....</h1>
  {:else if error}
    <h1 class="text- px-6 py-3 text-center text-4xl">Failed to login!</h1>
  {:else}
    <h1 class="text-text1 px-6 py-3 text-center text-4xl">You're logged-in!</h1>
  {/if}
</div>
