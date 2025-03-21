<script lang="ts">
	import { enhance } from '$app/forms';
	import Card from '$lib/components/card.svelte';
  import type { Database } from '$lib/database.types'; 
	import { slide } from 'svelte/transition';
  type Card = Database['public']['Tables']['cards']['Row'];

  let { data } = $props()
  let { cards } = $derived(data);

  let searchTerm: string = $state("");

  let filteredCards = $derived(
    cards.filter(card =>
      card.name.toLowerCase().includes(searchTerm.toLowerCase())
    )
  );

  type CardAndCount = {
    card: Card,
    count: number,
  }

  let deck: CardAndCount[] = $state([]);

  let totalDeckCount = $derived(
    deck.reduce((total, cardAndCount) => total + cardAndCount.count, 0)
  );
</script>

<div class="main">
  <div class="catalog-search-wrapper">
    <input type="text" name="search" id="" bind:value={searchTerm}>
    <h1>{totalDeckCount + " / 30"}</h1>
    <form action="?/createDeck" method="POST" use:enhance>
      <input style="display: none;" type="text" name="deck" bind:value={deck}>
      <button class="button primary" type="submit">CREATE DECK</button>
    </form>
    <div class="card-wrapper">
      {#each filteredCards as card}
         <!-- svelte-ignore a11y_consider_explicit_label -->
        <button class="invisible" onclick={()=>{
          if (!deck){ deck = [] }; // form submission makes deck null? so redefine it
          if(deck.length >= 30) return;
          
          const existingCard = deck.find(c => c.card.id === card.id);
          if (existingCard && existingCard?.count >= 2) return;

          if (existingCard) {
            existingCard.count++;
          } else {
            deck.push({ card: card, count: 1})
          }
        }}>
          <Card card={card}></Card>
        </button>
      {/each}
    </div>
  </div>
  <div class="deck">
    {#each deck as cardAndCount}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div transition:slide={{ axis: "x", duration: 250 }} class="card-in-deck-view" style="background-image: url({cardAndCount.card.image_url});" onclick={()=>{
      const cardIndex = deck.findIndex((c) => c.card.id === cardAndCount.card.id);
        if (cardIndex !== -1) {
          if(deck[cardIndex].count > 1){
            deck[cardIndex].count--;
          } else {
            deck.splice(cardIndex, 1);
          }
        }
    }}>
      <span class="name">{cardAndCount.card.name}</span><span class="count">x {cardAndCount.count}</span>
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