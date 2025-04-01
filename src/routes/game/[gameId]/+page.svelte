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

  let battlefieldElement: HTMLElement | null = $state(null);

  onMount(()=>{
    setCards(cards);
  })
</script>

<div id="game-frame">
  <div class="DEBUG">
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button disabled={ !$gameState.yourTurn } onclick={()=>{
      endTurn();
    }}>END TURN</button>
    <p>Game ID: {page.params.gameId}</p>
    <button onclick={()=> {enterFullscreen("game-frame")}}>Go Fullscreen</button>

    <div>Your turn: { !$gameState.yourTurn }</div>
  </div>
  <!-- GAME ZONES-->
  <div class="opponent-zone">
    <div class="opponent-graveyard"></div>
    <div class="opponent-hand-battlefield-zone">
      <div class="opponent-battlefield"></div>
      <div class="opponent-hand"></div>
    </div>
    <div class="opponent-deck">
      
    </div>
  </div>
  <div class="divider"></div>
  <div class="self-zone">
    <div class="self-graveyard">
      <Graveyard></Graveyard>
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
    grid-template-rows: 1fr 35%;
  }
  .self-hand{
    background-color: blueviolet;
  }
</style>