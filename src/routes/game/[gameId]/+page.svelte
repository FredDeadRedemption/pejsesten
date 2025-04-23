<script lang="ts">
  import { endTurn } from "$lib/socket/socket";
  import { page } from "$app/state";
  import { gameState } from "$lib/socket/socket";
	import Hand from "$lib/components/hand.svelte";
	import { onMount } from "svelte";
	import Battlefields from "$lib/components/battlefields.svelte";
	import Deck from "$lib/components/deck.svelte";
	import Graveyard from "$lib/components/graveyard.svelte";
	import { setCards } from "$lib/cards.js";
	import { enterFullscreen } from "$lib/util.js";

  let { data } = $props()
  let { cards } = $derived(data);

  // Pre-load all cards in existence
  onMount(()=>{
    setCards(cards);
  })
</script>

<div id="game-frame">
  <!-- DEBUGGING / LOGGIN -->
  <div class="DEBUG">
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button disabled={ !$gameState.yourTurn } onclick={()=>{
      endTurn();
    }}>END TURN</button>
    <p>Game ID: {page.params.gameId}</p>
    <button onclick={()=> {enterFullscreen("game-frame")}}>Go Fullscreen</button>

    <div>Your turn: { $gameState.yourTurn }</div>
  </div>
  <!-- GRAVEYARDS -->
  <div class="graveyard-zone">
    <div class="enemy-graveyard">
      <Graveyard bind:graveyard={ $gameState.enemy.graveyard }></Graveyard>
    </div>
    <div class="self-graveyard">
      <Graveyard bind:graveyard={ $gameState.self.graveyard }></Graveyard>
    </div>
  </div>
  <!-- HANDS & BATTLEFIELD -->
  <div class="hand-battlefield-zone">
    <div class="enemy-hand">
      
      <Hand bind:hand={ $gameState.enemy.hand}></Hand>
    </div>
    <div class="battlefield">
      <Battlefields 
      bind:selfBattleField={ $gameState.self.battlefield } 
      bind:enemyBattleField={ $gameState.enemy.battlefield }
      bind:selfHP={ $gameState.self.hp }
      bind:enemyHP={ $gameState.enemy.hp }
      ></Battlefields>
    </div>
    <div class="self-hand">
      <Hand bind:hand={ $gameState.self.hand }></Hand>
    </div>
  </div>
  <!-- DECKS -->
  <div class="deck-zone">
    <div class="enemy-deck">
      <Deck bind:deck={ $gameState.enemy.deck }></Deck>
    </div>
    <div class="self-deck">
      <Deck bind:deck={ $gameState.self.deck }></Deck> 
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
  #game-frame {
    min-width: 100%;
    min-height: calc(100vh - 50px);
    max-height: calc(100vh - 50px);
    display: grid;
    grid-template-columns: 15% 1fr 15%;
  }
  .graveyard-zone, .deck-zone{
    display: grid;
    grid-template-rows: 1fr 1fr;
  }
  .self-deck, .self-graveyard{
    background-color: red;
  }
  .enemy-deck, .enemy-graveyard{
    background-color: goldenrod;
  }
  .hand-battlefield-zone{
    display: grid;
    grid-template-rows: 18% 1fr 18%;
  }
  .battlefield{
    background-color: burlywood;
    display: grid;
    grid-template-rows: 1fr 1fr;
  }
  .self-hand, .enemy-hand{
    background-color: blueviolet;
  }
  .enemy-hand, .enemy-deck{ //TODO: det her er måske kun temporary fix
    pointer-events: none;
  }
</style>