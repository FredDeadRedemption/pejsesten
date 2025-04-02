<script lang="ts">
	import Card from '$lib/components/card.svelte';
	import CardAdminPanel from '$lib/components/cardAdminPanel.svelte';
	import { getIcon } from '$lib/icons.js';

  let { data } = $props()
  let { land_enums, profile, supabase } = $derived(data);
  let { cards } = $state(data);

  let searchTerm: string = $state("");

  let filteredCards = $derived(
    cards.filter(card =>
      card.name.toLowerCase().includes(searchTerm.toLowerCase())
    )
  );

  const onDeleteCard = (id: number) => cards = cards.filter(card => card.id != id);

  type CostMap = {
    [key: string]: { color: string; icon: string };
  };

  const costMap: CostMap = {
    earth_land: { color: "#28b84a", icon: "leaf" }, // Earth
    holy_land: { color: "#f3ca12", icon: "cross" }, // Holy
    dream_land: { color: "#af1cb6", icon: "lily" }, // Dream
    death_land: { color: "#2a2929", icon: "skull" } // Death
  };
</script>

<main class="main">
  <input type="text" name="search" id="" bind:value={searchTerm}>
  <div class="costs">
    {#each land_enums as c}
      <p>{c.name.replace("_land", "")}</p>
      <div class="cost" style="background-color: {costMap[c.name].color};">
      <span class="icon">{@html getIcon(costMap[c.name].icon)}</span>
      </div>
    {/each}
  </div>
  <h1>Cards:</h1>
  <div class="card-wrapper">
    {#each filteredCards as card (card.id)}
      <div class="card-admin-panel-wrapper">
        <div id={card.name}>
          <Card card={card}></Card>
        </div>
        {#if profile?.is_admin}
          <CardAdminPanel card={card} supabase={supabase} onDeleteCard={onDeleteCard}></CardAdminPanel>
        {/if}
      </div>
    {/each}
  </div>
</main>
 
<style lang="scss">
  .main{
    margin: 30px;
  }
  .card-wrapper{
    padding: 10px;
    background-color: $grey-mid;
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
  .costs{
    width: 100%;
    display: flex;
    align-items: center;
    gap: 2px;
    width: 100%;
    //background-color: $grey-light;
    padding: 2px;
    .cost{
      color: $white;
      display: flex;
      justify-content: center;
      align-items: center;
      width: 14px;
      height: 14px;
      border-radius: 100px;
      .icon{
        display: flex;
        justify-content: center;
        align-items: center;
      }
    } 
  }
</style>