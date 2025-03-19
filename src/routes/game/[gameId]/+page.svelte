<script>
   import { endTurn } from "$lib/socket/socket";

  import { page } from "$app/state";
	import { browser } from "$app/environment";

  import { yourTurn } from "$lib/socket/socket";

  let gameId = $state(page.params.gameId);

  function enterFullscreen() {
    if(!browser) return;
    const elem = document.getElementById("fullscreen-div");
    if (elem?.requestFullscreen) {
      elem.requestFullscreen();
    }
  }
</script>

<div id="fullscreen-div">
  <!-- svelte-ignore a11y_consider_explicit_label -->
  <button disabled={ !$yourTurn } class="button primary" onclick={()=>{
    endTurn();
  }}>END TURN</button>
  <h1>Game ID: {gameId}</h1>
</div>
<button onclick={()=> {enterFullscreen()}}>Go Fullscreen</button>

<div>Your turn: { $yourTurn }</div>

<style lang="scss">
  #fullscreen-div {
    width: 100%;
    height: 100%;
    background-color: lightgreen;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 2rem;
  }
</style>