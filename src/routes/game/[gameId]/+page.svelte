<script>
   import { endTurn } from "$lib/socket/socket";

  import { page } from "$app/state";
	import { browser } from "$app/environment";

  import { yourTurn } from "$lib/socket/socket";
	import Hand from "$lib/components/hand.svelte";

  let gameId = $state(page.params.gameId);

  function enterFullscreen() {
    if(!browser) return;
    const elem = document.getElementById("fullscreen-div");
    if (elem?.requestFullscreen) {
      elem.requestFullscreen();
    }
  }

  let hand = $state([12, 23, 44]);
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
  <div class="opponent-zone">
    <div class="opponent-graveyard"></div>
    <div class="opponent-hand-battlefield-zone">
      <div class="opponent-battlefield"></div>
      <div class="opponent-hand"></div>
    </div>
    <div class="opponent-deck"></div>
  </div>
  <div class="divider"></div>
  <div class="self-zone">
    <div class="self-graveyard"></div>
    <div class="self-hand-battlefield-zone">
      <div class="self-battlefield"></div>
      <div class="self-hand">
        <Hand hand={hand}></Hand>
      </div>
    </div>
    <div class="self-deck">
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <button class="button primary" onclick={()=>{hand.push(2)}}></button>
    </div>
  </div>
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
    border: 2px solid $black;
    min-width: 100%;
    min-height: calc(100vh - 50px);
    display: grid;
    grid-template-rows: 1fr 2px 1fr;
  }
  .opponent-zone{
    background-color: lightsalmon;
  }
  .divider{
    background-color: $black;
    width: 100%;
  }
  .self-zone{
    background-color: teal;
    display: grid;
    grid-template-columns: 15% 1fr 15%;
  }
  .self-deck{
    background-color: red;
  }
  .self-graveyard{
    background-color: red;
  }
  .self-hand-battlefield-zone{
    display: grid;
    grid-template-rows: 1fr 1fr;
  }
  .self-hand{
    background-color: blueviolet;
  }
</style>