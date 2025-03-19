<script>
   import { endTurn } from "$lib/socket/socket";

  import { page } from "$app/state";
  let gameId = $state(page.params.gameId);
</script>

<!-- svelte-ignore a11y_consider_explicit_label -->
<button class="button primary" onclick={()=>{
  endTurn();
}}>END TURN</button>
<h1>Game ID: {gameId}</h1>