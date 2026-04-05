<script lang="ts">
  import { queueUp, leaveQueue, invalidateSocket, connectSocket } from "$lib/socket/socket";
  import type { PlayerMetaData } from "$lib/shared/types";
	import { fly } from "svelte/transition";

  let decks: any[] = [
    {
      name: "Deck 1",
      cards: [1,2,3,4,5,6,7,8,9,10]
    }
  ]

  let choosenDeckJson = $state([1,2,3,4,5,6,7,8,9,10]);
    
  let dev = $state(false)

  let url = $derived("http://localhost:3002/");

  $effect(()=>{
    console.log("url changed invalidating socket " + url) 
    invalidateSocket();
  })

  let playerMetaData: PlayerMetaData = $derived({
    username: "Out-of-Towner",
    choosenDeck: choosenDeckJson, 
    avatar: "uaogidsogijsogij"
  })

  let ellipseVar = $state('.');
  setInterval(() => ellipseVar = ellipseVar.length >= 3 ? '.' : ellipseVar + '.', 300);

  let queuedUp = $state(false);

  const resetProd = async () => {
    const res = await fetch("https://matrixz-gs.up.railway.app/resetGame");
   
    console.log(res);
  }
</script>

<main class="main">
  <p>Runnin on <strong>{dev ? "Development" : "Production"}</strong></p>
  <p>Playin as <strong>{"random troldmayn"}</strong></p>
  <button onclick={resetProd}>reset prod gamestate & queueu</button>
  <input type="checkbox" name="url" id="" bind:checked={dev}>

  <select bind:value={choosenDeckJson}>
    {#each decks as deck}
      <option value={deck.cards}>{deck.name}</option>
    {/each}
  </select>

  <button class="button primary" class:queuedUp={queuedUp} onclick={() => { 
    connectSocket(url);
    queuedUp ? leaveQueue() : queueUp(playerMetaData);
    queuedUp = true;
    }}>
      {queuedUp ? `Queueing ${ellipseVar}` : "Join Queue"}
  </button>
  {#if queuedUp}
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button class="button primary" transition:fly={{ duration: 250 }} onclick={() => { 
      leaveQueue();
      queuedUp = false;
      }}>
      Leave Queue
    </button>
  {/if}
</main>

<style lang="scss">
  .queuedUp{
    background-color: $grey-mid;
    &:hover{
      background-color: $grey-mid;
      cursor: auto;
    }
  }
  .main {
    margin: 30px;
    display: flex;
    flex-direction: column;
    width: 100%;
    align-items: center;
    text-align: center;
    gap: 10px;
  }
</style>
