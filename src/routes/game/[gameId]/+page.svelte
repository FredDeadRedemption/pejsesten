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
  <div class="DEBUG">
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button disabled={ !$yourTurn } onclick={()=>{
      endTurn();
    }}>END TURN</button>
    <p>Game ID: {gameId}</p>
    <button onclick={()=> {enterFullscreen()}}>Go Fullscreen</button>

    <div>Your turn: { $yourTurn }</div>
  </div>
  <!-- GAME ZONES-->
  <div class="opponent-half"></div>
  <div class="self-half"></div>
</div>

<style lang="scss">
  .DEBUG{
    position: absolute;
    background-color: $black;
    color: $white;
    border: 2px solid red;
    top: 0;
    width: 90%;
    display: flex;
    flex-direction: row;
    transform: translateY(50px);
    font-size: 1rem;
    justify-content: space-between;
  }
  #fullscreen-div {
    border: 2px solid red;
    min-width: 100%;
    min-height: calc(100vh - 50px);
    display: grid;
    grid-template-rows: 1fr 1fr;
  }
  .opponent-half{
    background-color: lightsalmon;
  }
  .self-half{
    background-color: teal;
  }
</style>