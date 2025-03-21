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

  let deck: number[] = $state([]);
</script>

<input type="text" name="search" id="" bind:value={searchTerm}>
<div class="main">
  <div class="card-wrapper">
    {#each filteredCards as card}
       <!-- svelte-ignore a11y_consider_explicit_label -->
      <button class="invisible" onclick={()=>{
        if (!deck){ deck = [] }; // form submission makes deck null? so redefine it
        if(deck.filter(cardId => cardId === card.id).length >= 3) return;
        deck.push(card.id)
      }}>
        <Card card={card}></Card>
      </button>
    {/each}
  </div>
  <div class="deck">
    <h1>{deck.length + " / 45"}</h1>
    {#each deck as cardId}
      <p>{cardId}</p>
    {/each}
    <form action="?/createDeck" method="POST" use:enhance>
    <input style="display: none;" type="text" name="deck" bind:value={deck}>
    <button class="button primary" type="submit">CREATE DECK</button>
    </form>
  </div>
</div>


<style lang="scss">
  .deck{
    max-height: 450px;
    display: flex;
    flex-direction: column;
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
  .deck{
    padding: 10px;
    background-color: $grey-mid;
    border-radius: 10px;
    display: flex;
    flex-wrap: wrap;
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