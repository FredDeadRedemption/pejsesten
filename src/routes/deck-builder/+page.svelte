<script lang="ts">
	import { enhance } from '$app/forms';
	import Card from '$lib/components/card.svelte';
  import type { Database } from '$lib/database.types'; 
	import { slide } from 'svelte/transition';

  let { data } = $props()
  let { cards } = $derived(data);

  let searchTerm: string = $state("");

  let filteredCards = $derived(
    cards.filter(card =>
      card.name.toLowerCase().includes(searchTerm.toLowerCase())
    )
  );

  const getCardData = (id: number) => cards.find((card) => card.id === id);

  let decks = $state(data?.decks);
  let deckId: number | null = $state(null);
  let deck: number[] = $state([]); // contains id's of all cards
  let deckUniques: number[] = $derived([...new Set(deck)]);; // contains id's of all cards (no duplicates)

  // Serialize the deck array to JSON whenever it changes
  let deckJSON = $state('');
  $effect(() => {
    deckJSON = JSON.stringify(deck);
  });

  const loadDeck = (selectedDeckId: number) => {
    const selectedDeck = decks.find((d) => d.id === selectedDeckId);
    if (selectedDeck) {
      deckId = selectedDeck.id;
      if (Array.isArray(selectedDeck.cards)) {
        deck = selectedDeck.cards.map((cardId) => Number(cardId)); // Convert each item to a number
      } else {
        deck = []; // Fallback to an empty array if `cards` is not an array
      }
   } else {
      deckId = null;
      deck = []; // Reset the deck if "New Deck" is selected
    }
  }
</script>

<div class="main">
  <div class="catalog-search-wrapper">
    <input type="text" name="search" id="" bind:value={searchTerm}>
    <h1>CURRENT DECK ID: {deckId || "null"}</h1>
    <h1>{deck.length + " / 30"}</h1>
    <form action="?/createDeck" method="POST" use:enhance={() => {
      return async ({ result }: any) => {

        console.log(result)
        deckId = result?.data?.newDeck?.id;
        if (decks.some((deck) => deck.id === result?.data?.newDeck?.id)) return;
        decks.push(result?.data?.newDeck);
      };
      }}>
      <input type="hidden" name="deckId" bind:value={deckId}>
      <input type="hidden" name="deck" bind:value={deckJSON}>
      <button class="button primary" type="submit">CREATE DECK</button>
    </form>
    <div class="card-wrapper">
      {#each filteredCards as card}
         <!-- svelte-ignore a11y_consider_explicit_label -->
        <button class="invisible" onclick={()=>{
          if(!deck){ deck = [] }; // form submission makes deck null? so redefine it (TODO: remove later)
          if(deck.length >= 30) return; // deck cant have more than 30 cards
          if(deck.filter((c) => c === card.id).length >= 2) return; // deck cant have more than 2 of each

          deck.push(card.id)
        }}>
          <Card card={card}></Card>
        </button>
      {/each}
    </div>
  </div>
  <div class="deck">
    <select name="decks" id="decks" onchange={(e: any) => {
      const selectedDeckId = parseInt(e?.target?.value);
      loadDeck(selectedDeckId); // Load the selected deck's cards
    }}>
      {#each decks as deck}
        <option value={deck.id}>{deck.id}</option>
      {/each}
      <option value="">New Deck</option>
    </select>
    {#each deckUniques as id}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div transition:slide={{ axis: "x", duration: 250 }} class="card-in-deck-view" style="background-image: url({getCardData(id)?.image_url});" onclick={()=>{
      const cardIndex = deck.findIndex((c) => c === id);
      if (cardIndex !== -1) deck.splice(cardIndex, 1);
    }}>
      <span class="name">{getCardData(id)?.name}</span><span class="count">x {deck.filter((c) => c === id).length}</span>
    </div>
    {/each}
  </div>
</div>


<style lang="scss">
  .deck{
    background-color: $grey-mid;
    border-radius: 3px;
    overflow: hidden;
    display: flex;
    flex-wrap: wrap;    
    max-height: 500px;
    flex-direction: column;
    .card-in-deck-view{
      display: flex;
      justify-content: space-between;
      align-items: center;
      height: 40px;
      width: 100%;
      background-size: cover;
      color: $white;
      user-select: none; 
      font-weight: 800;
      &:hover{
        cursor: pointer;
      }
      .name{
        display: flex;
        align-items: center;
        white-space: nowrap;
        flex-grow: 1;
        padding-left: 10px;
        height: 100%;
        background: linear-gradient(to right, 
        rgb(0, 0, 0) 0%, 
        rgba(31, 31, 31, 0)100%);
      }
      .count{
        display: flex;
        align-items: center;
        justify-content: flex-end;
        white-space: nowrap;
        width: 50px;
        padding-right: 10px;
        height: 100%;
        background: linear-gradient(to left, 
        rgb(0, 0, 0) 0%, 
        rgba(31, 31, 31, 0)100%);
      }
    }
  }
  .invisible{
    box-sizing: auto !important;
    border: none;
    background-color: transparent;
    border-radius: 5px;
    &:hover{
      scale: 1.04;
      z-index: 3;
      cursor: pointer;
    }
  }
  .main{
    display: grid;
    grid-template-columns: 1fr 250px;
    gap: 10px;
  }
  .card-wrapper{
    padding: 10px;
    background-color: $grey-mid;
    border-radius: 10px;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
</style>