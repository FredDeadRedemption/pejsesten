import { writable, derived } from "svelte/store";
import { setSocketURL } from "$lib/socket/socketStore";

const toggleState = writable(false); // false = Development, true = Production

const toggleLabel = derived(toggleState, ($toggleState) =>
  $toggleState ? "Production" : "Development"
);

toggleState.subscribe(($toggleState) => {
  setSocketURL($toggleState); // Update the socket URL when toggleState changes
});

export { toggleState, toggleLabel };