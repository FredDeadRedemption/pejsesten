<script lang="ts">
	import { enhance } from '$app/forms';
	import Card from '$lib/components/card.svelte';
	import Filter from '$lib/components/filter.svelte';
	import { getIcon } from '$lib/icons.js';
	import { getRandomDeckName, getRandomString } from '$lib/util.js';
	import { slide } from 'svelte/transition';
  import type { Card as CardT } from '$lib/shared/types';

  let filteredCards = $state<CardT[]>([]);

  type Deck = {
    id: number;
    name: string;
    cards: number[]; // array of card id's
  }

  let cards = $state<CardT[]>([]); // TODO: move this to a load function and make it so it only fetches id and name for better performance

  const getCardData = (id: number) => cards.find((card) => card.id === id);

  let decks = $state<Deck[]>([]);
  let selectedDeckID: number | null = $state(null);
  let deck: number[] = $state([]); // contains id's of all cards
  let deckUniques: number[] = $derived([...new Set(deck)]);; // contains id's of all cards (no duplicates)
  let selectedDeckName: string | null = $state(null);
  let inspectingDeck: boolean = $state(false);

  // Serialize the deck array to JSON whenever it changes
  let deckJSON = $state('');
  $effect(() => {
    deckJSON = JSON.stringify(deck);
  });

  const deleteDeck = (deckId: number | null) => {
    // TODO: implement
  }

  const loadExistingDeck = (selectedDeckId: number) => {
    const selectedDeck = decks.find((d) => d.id === selectedDeckId);
    if (selectedDeck) {
      selectedDeckID = selectedDeck.id;
      selectedDeckName = selectedDeck.name;
      inspectingDeck = true;
      if (Array.isArray(selectedDeck.cards)) {
        deck = selectedDeck.cards.map((cardId: number) => Number(cardId)); // Convert each item to a number
      } else {
        deck = []; // Fallback to an empty array if `cards` is not an array
      }
   } else {
      selectedDeckID = null;
      deck = []; // Reset the deck if "New Deck" is selected
    }
  }

  const loadNewDeck = () => {
    inspectingDeck = true;
    selectedDeckName = getRandomDeckName();
    deck = [];
  }

  let ellipseVar = $state('.');
  setInterval(() => ellipseVar = ellipseVar.length >= 3 ? '.' : ellipseVar + '.', 300);

  let savingDeck: boolean = $state(false); // TODO: lav en loading ting så man kan se at den sletter et deck
</script>

<div class="main">
  <div class="catalog-search-wrapper">
    <Filter bind:cards={cards} bind:filteredCards={filteredCards}></Filter>
    <div class="card-wrapper">
      {#each filteredCards as card (card.id)}
         <!-- svelte-ignore a11y_consider_explicit_label -->
        <button class="invisible" onclick={()=>{
          if(!deck){ deck = [] }; // form submission makes deck null? so redefine it (TODO: remove later)
          if(deck.length >= 50) return; // deck cant have more than 30 cards
          //if(deck.filter((c) => c === card.id).length >= 2) return; // deck cant have more than 2 of each

          deck.push(card.id) 
        }}>
          <Card card={card}></Card>
        </button>
      {/each}
    </div>
  </div>
  <div class="deck">
    <!-- RENDER CARDS IN SELECTED DECK -->
    {#if inspectingDeck}
      <div class="name-length">
        <input id="name-input" type="text" maxlength="36" bind:value={selectedDeckName}>
        <span class="length">{deck.length} / 50</span>
      </div>
      <div class="cards">
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
      <div class="back-delete-btn-grp">
        <form id="back-form" action="?/createDeck" method="POST" use:enhance={() => {
          return async ({ result }: any) => {
            selectedDeckID = null;
            selectedDeckName = null;
            inspectingDeck = false;
            savingDeck = false;
            console.log(result)
            const existingDeck = decks?.find((deck) => deck.id === result?.data?.newDeck?.id);
            if (existingDeck) {
                // Update the existing deck's cards
                existingDeck.cards = result?.data?.newDeck?.cards;
                // Also update the name if it might change
                existingDeck.name = result?.data?.newDeck?.name || existingDeck?.name;
            } else {
                // Only push if it's a genuinely new deck
                decks.push(result.data.newDeck);
            }
          };
          }}>
          <input type="hidden" name="deckId" bind:value={selectedDeckID}>
          <input type="hidden" name="deck" bind:value={deckJSON}>
          <input type="hidden" name="name" bind:value={selectedDeckName}>
          <button class="button primary back" type="submit" onclick={() => {savingDeck = true}} class:saving={savingDeck}>{savingDeck ? `Saving Deck ${ellipseVar}` : "Done"}</button>
        </form>
        <button class="delete" onclick={() => deleteDeck(selectedDeckID)}><span class="icon">{@html getIcon("delete")}</span></button>
      </div>
      {:else}
     <!-- RENDER ALL USERS DECKS -->
      {#each decks as deck}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div transition:slide={{ axis: "x", duration: 250 }} class="deck-in-deck-view" style="background-image: url(TODO MAKE SOME KINDA IMAGE HERE);" onclick={() => {
          loadExistingDeck(deck.id);
        }}>
          <span class="name">{deck?.name}</span>
        </div>
      {/each}
      <button class="button primary new" onclick={() => {
        loadNewDeck()
      }}>New Deck</button>
    {/if}
  </div>
</div>


<style lang="scss">
  .name-length{
    display: grid;
    grid-template-columns: 1fr 40px;
    .length{
      display: flex;
      font-size: 0.8rem;
      align-items: center;
      justify-content: center;
      background-color: $secondary;
      color: $white;
    }
  }
  .catalog-search-wrapper{
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .deck{
    background-color: $grey-light;
    border-radius: 3px;
    overflow: scroll;
    display: flex;    
    height: 500px;
    flex-direction: column;
    .cards{
      overflow: scroll;
    }
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
    .deck-in-deck-view{
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
        overflow: hidden;        
        text-overflow: ellipsis;   
        flex-grow: 1;
        padding-left: 10px;
        height: 100%;
        background: linear-gradient(to right, 
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
  #back-form{
    margin-top: auto;
  }
  #name-input{
    height: 40px;
    font-weight: bold;
    background-color: $primary;
    border: none;
    color: $white;
    text-align: center;
    padding: 5px;
    &:focus{
      outline: none;
    }
  }
  .new{
    margin-top: auto;
    min-width: 100%;
  }
  .back{
    min-width: 100%;
    border-top-left-radius: 0px;
    border-bottom-right-radius: 0px;
    border-top-right-radius: 0px;
    &.saving{
      background-color: $grey-mid;
      &:hover{
        background-color: $grey-mid;
        cursor: auto;
      }
    }
  }
  .delete{
    background-color: $secondary;
    border: none;
    color: $white;
    &:hover{
      background-color: $black;
      cursor: pointer;
    }
  }
  .back-delete-btn-grp{
    margin-top: auto;
    display: grid;
    grid-template-columns: 1fr 35px;
  }
  .main{
    margin: 30px;
    display: grid;
    grid-template-columns: 1fr 250px;
    gap: 10px;
  }
  .card-wrapper{
    padding: 10px;
    background-color: $grey-light;
    border-radius: 10px;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
</style>