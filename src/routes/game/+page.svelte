<script lang="ts">
  import { queueUp, leaveQueue, invalidateSocket, connectSocket } from "$lib/socket/socket";
  import type { Database, Json } from '$lib/database.types'; 
	import { fly, slide } from "svelte/transition";
  type Deck = Database['public']['Tables']['decks']['Row'];

  let { data } = $props()
  let { profile, decks } = $derived(data)
    
  let dev = $state(false)

  let url = $derived(dev ? "http://localhost:3000/" : "https://matrixz-gs.up.railway.app/");

  $effect(()=>{
    console.log("url changed invalidating socket " + url) 
    invalidateSocket();
  })

  let choosenDeckJson: Json | null = $state(null)
  let choosenDeck: number[] = $derived(
    choosenDeckJson ?? []
  );
  // share this to the backend
  type PlayerMetaData = {
    username: string,
    choosenDeck: Number[],
    avatar: string,
  }

  let playerMetaData: PlayerMetaData = $derived({
    username: profile?.username ?? "Out-of-Towner",
    choosenDeck: choosenDeck, 
    avatar: "uaogidsogijsogij"
  })

  let dots = $state('.');
  let intervalId: number;

  let ellipseVar = $state('.');
  setInterval(() => ellipseVar = ellipseVar.length >= 3 ? '.' : ellipseVar + '.', 300);

  let queuedUp = $state(false);
</script>

<main class="main">
  <a href="/game/test">Load Test Game (no session)</a>
  <p>Runnin on <strong>{dev ? "Development" : "Production"}</strong> change in <a href="/settings">settings</a></p>
  <p>Playin as <strong>{profile?.username}</strong></p>
  <input type="checkbox" name="url" id="" bind:checked={dev}>

  <select bind:value={choosenDeckJson}>
    {#each decks as deck}
      <option value={deck.cards}>{deck.id}</option>
    {/each}
  </select>

  <button class="button primary" class:queuedUp={queuedUp} onclick={() => { 
    connectSocket(url);
    queuedUp ? leaveQueue() : queueUp(profile?.username || "Out-of-Towner");
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
  a{
    color: $primary;
  }
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
