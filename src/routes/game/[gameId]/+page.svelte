<script lang="ts">
  import { endTurn } from "$lib/socket/socket";
  import { page } from "$app/state";
	import { browser } from "$app/environment";
  import { gameState } from "$lib/socket/socket";
	import Hand from "$lib/components/hand.svelte";
	import { onMount } from "svelte";
	import Battlefield from "$lib/components/battlefield.svelte";
	import Deck from "$lib/components/deck.svelte";
	import Graveyard from "$lib/components/graveyard.svelte";
	import { setCards } from "$lib/cards.js";
	import { enterFullscreen } from "$lib/util.js";

  let { data } = $props()
  let { cards } = $derived(data);

  let battlefieldElement: HTMLElement | null = $state(null); // TODO: get rid of this shit

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
  <!-- GAME ZONES-->
  <div class="enemy-zone">
    <div class="enemy-graveyard">
      <Graveyard bind:graveyard={ $gameState.enemy.graveyard }></Graveyard>
    </div>
    <div class="enemy-hand-battlefield-zone">
      <div class="enemy-hand">
        <Hand bind:hand={ $gameState.enemy.hand} battleFieldElement={battlefieldElement}></Hand>
      </div>
      <div class="enemy-battlefield">
        <Battlefield bind:battleField={ $gameState.enemy.battlefield }></Battlefield>
      </div>
    </div>
    <div class="enemy-deck">
      <Deck bind:deck={ $gameState.enemy.deck }></Deck>
    </div>
  </div>
  <div class="divider"></div>
  <div class="self-zone">
    <div class="self-graveyard">
      <Graveyard bind:graveyard={ $gameState.self.graveyard }></Graveyard>
    </div>
    <div class="self-hand-battlefield-zone">
      <div class="self-battlefield" bind:this={battlefieldElement}>
        <Battlefield bind:battleField={ $gameState.self.battlefield }></Battlefield>
      </div>
      <div class="self-hand">
        <Hand bind:hand={ $gameState.self.hand } battleFieldElement={battlefieldElement}></Hand>
      </div>
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
    border: 2px solid $black;
    min-width: 100%;
    min-height: calc(100vh - 50px);
    max-height: calc(100vh - 50px);
    display: grid;
    grid-template-rows: 1fr 2px 1fr;
  }
  .enemy-zone{
    background-color: lightsalmon;
  }
  .divider{
    background-color: $black;
    width: 100%;
  }
  .self-zone, .enemy-zone{
    background-color: teal;
    display: grid;
    grid-template-columns: 15% 1fr 15%;
  }
  .self-deck, .enemy-deck{
    background-color: red;
  }
  .self-graveyard, .enemy-graveyard{
    background-color: red;
  }
  .self-hand-battlefield-zone{
    display: grid;
    grid-template-rows: 1fr 35%;
  }
  .enemy-hand-battlefield-zone{
    display: grid;
    grid-template-rows: 35% 1fr;
  }
  .self-hand, .enemy-hand{
    background-color: blueviolet;
  }
  .enemy-hand, .enemy-deck{ //TODO: det her er måske kun temporary fix
    pointer-events: none;
  }
</style>