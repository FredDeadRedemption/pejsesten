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
  <div class="bar-wrapper"> 
    <input type="text" name="search" id="" placeholder="Search Catalog" bind:value={searchTerm}>
    <div class="costs">
      {#each land_enums as c}
        <span class="text">{c.name.replace("_land", "")}</span>
        <div class="cost" style="background-color: {costMap[c.name].color};">
        <span class="icon">{@html getIcon(costMap[c.name].icon)}</span>
        </div>
      {/each}
    </div>
  </div>
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
  input{  
      border: none;
      border: 1px solid $grey-mid;
      background-color: $grey-ultralight;  
      outline: none;
      color: $grey-ultradark;
      border-radius: 5px;
      padding: 10px;
    }
  .main{
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin: 30px;
  }
  .bar-wrapper{
    padding: 10px;
    background-color: $grey-light;
    border-radius: 10px;
    display: flex;
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
    .text{
      color: $grey-dark;
      margin: 5px;
    } 
  }
</style>