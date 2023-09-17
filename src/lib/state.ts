import type { Writable } from "svelte/store";
import type { State } from "./gen/types";
import { writable } from "svelte/store";
import { getContext, setContext } from "svelte";
import { invoke } from "@tauri-apps/api";

export const state: Writable<State | undefined> = writable(undefined);

export const STATE_CONTEXT_KEY = "State";

export const getStateContext = (): State => {
  return getContext(STATE_CONTEXT_KEY);
};

export const setStateContext = () => {
  setContext(STATE_CONTEXT_KEY, state);
};

export const loadState = async () => {
  let loadedState: State = await invoke("load_state");

  state.set(loadedState);

  state.subscribe(async (state) => {
    console.log(state);
    if (state)
      await invoke("save_state", {
        state,
      });
  });
};
