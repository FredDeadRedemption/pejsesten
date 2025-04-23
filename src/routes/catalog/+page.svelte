<script lang="ts">
	import Card from '$lib/components/card.svelte';
	import CardAdminPanel from '$lib/components/cardAdminPanel.svelte';
	import { getIcon } from '$lib/icons.js';
  import type { Database } from '$lib/database.types'; 
	import Filter from '$lib/components/filter.svelte';
  type CardT = Database['public']['Tables']['cards']['Row'];

  let { data } = $props()
  let { profile, supabase } = $derived(data);
  let { cards } = $state(data);

  let filteredCards = $state<CardT[]>([]);

  const onDeleteCard = (id: number) => { cards = cards.filter(card => card.id != id); filteredCards = cards;};
  const onUpdateCard = (updatedCard: CardT) => {
    let i = cards.findIndex(card => card.id === updatedCard.id);
    cards[i] = updatedCard;
    filteredCards = cards;
  }
</script>

<main class="main">
  <Filter bind:cards={cards} bind:filteredCards={filteredCards}></Filter>
  <div class="card-wrapper">
    {#each filteredCards as card (card.id)}
      <div class="card-admin-panel-wrapper">
        <div id={card.name}>
          <Card card={card}></Card>
        </div>
        {#if profile?.is_admin}
          <CardAdminPanel card={card} supabase={supabase} onDeleteCard={onDeleteCard} onUpdateCard={onUpdateCard}></CardAdminPanel>
        {/if}
      </div>
    {/each}
  </div>
</main>
 
<style lang="scss">
  .main{
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin: 30px;
  }
  .card-wrapper{
    padding: 10px;
    background-color: $grey-light;
    border-radius: 10px;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
  .card-admin-panel-wrapper{
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
</style>