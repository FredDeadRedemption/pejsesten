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

  const onDeleteCard = (id: number) => cards = cards.filter(card => card.id != id);
  const onUpdateCard = (updatedCard: CardT) => {
    let i = cards.findIndex(card => card.id === updatedCard.id);
    cards[i] = updatedCard;
  }
 
  type ManaMap = {
    [key: string]: { color: string; icon: string, iconColor: string };
  };

  const manaMap: ManaMap = {
    green: { color: " #38761d", icon: "manaGreen", iconColor: "#f3f3f3" }, // Earth
    white: { color: "#e9e9e9", icon: "manaWhite", iconColor: "#434343" }, // Holys
    purple: { color: "#474ea7", icon: "manaPurple", iconColor: "#f3f3f3" }, // Dream
    black: { color: "#434343", icon: "manaBlack", iconColor: "#f3f3f3" }, // Death
    red: { color: "#cc0000", icon: "manaRed", iconColor: "#f3f3f3" },
    orange: { color: "#bc874f", icon: "manaOrange", iconColor: "#f3f3f3" }
  };
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