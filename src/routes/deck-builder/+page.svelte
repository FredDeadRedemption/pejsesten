<script lang="ts">
	import { enhance } from '$app/forms';
	import Card from '$lib/components/card.svelte';

  let { data } = $props()
  let { cards } = $derived(data);

  let searchTerm: string = $state("");

  let filteredCards = $derived(
    cards.filter(card =>
      card.name.toLowerCase().includes(searchTerm.toLowerCase())
    )
  );

  type DeckCard = {
    id: number,
    count: number,
    name: string,
    image_url: string
  }

  let deck: number[] = $state([]);
  let deckMetaData: DeckCard[] = $state([]);
</script>

<input type="text" name="search" id="" bind:value={searchTerm}>
<h1>{deck.length + " / 30"}</h1>
<form action="?/createDeck" method="POST" use:enhance>
  <input style="display: none;" type="text" name="deck" bind:value={deck}>
  <button class="button primary" type="submit">CREATE DECK</button>
  </form>
<div class="main">
  <div class="card-wrapper">
    {#each filteredCards as card}
       <!-- svelte-ignore a11y_consider_explicit_label -->
      <button class="invisible" onclick={()=>{
        if (!deck){ deck = [] }; // form submission makes deck null? so redefine it
        if(deck.filter(cardId => cardId === card.id).length >= 3) return;
        deck.push(card.id)

        const existingCard = deckMetaData.find(deckCard => deckCard.id === card.id);
        if (existingCard) {
          existingCard.count += 1;
        } else {
          deckMetaData.push({ id: card.id, count: 1, name: card.name, image_url: card.image_url });
        }
      }}>
        <Card card={card}></Card>
      </button>
    {/each}
  </div>
  <div class="deck">
    {#each deckMetaData as card}
    <div class="card-in-deck-view">
      <!-- <img src={card.image_url} alt=""> -->
      <!-- <div class="card-in-deck-content"> -->
        <p>{card.id}</p><p> COUNT: {card.count}</p> 
      <!-- </div> -->
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
    max-height: 450px;
    flex-direction: column;
    .card-in-deck-view{
      display: flex;
      justify-content: center;
      align-items: center;
      height: 40px;
      width: 100%;
      img{
        width: 100%;
        height: 100%;
        object-fit: cover;
      }
    }
    .card-in-deck-content{
      position: absolute;
      height: 100%;
      width: 100%;
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