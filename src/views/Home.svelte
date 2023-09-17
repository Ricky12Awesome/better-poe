<script lang="ts">
  import { getSettingsContext } from "../lib/settings";
  import type { Token } from "../lib/gen/types";
  import { invoke } from "@tauri-apps/api";

  let settings = getSettingsContext();
  let getTokenPromise: Promise<Token> = undefined;

  const getToken = () => {
    getTokenPromise = invoke("get_token").then((token: Token) => {
      $settings.token = token;
    });
  };
</script>

<div class="pt-[45vh] text-center">
  {#if !$settings.token && !getTokenPromise}
    <button
      on:click={getToken}
      class="text-text2 bg-primary px-6 py-3 text-center text-4xl hover:bg-secondary"
      >Login
    </button>
  {:else if getTokenPromise && !$settings.token}
    <h1 class="text-text1 px-6 py-3 text-center text-4xl">Logging in....</h1>
  {:else}
    <h1 class="text-text1 px-6 py-3 text-center text-4xl">
      You're now logged-in!
    </h1>
  {/if}
</div>
