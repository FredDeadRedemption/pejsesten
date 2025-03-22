<script lang="ts">
	import Card from '$lib/components/card.svelte';
	import { getIcon } from '$lib/icons.js';

  let { data } = $props()
  let { land_enums, cards } = $derived(data);

  let searchTerm: string = $state("");

  let filteredCards = $derived(
    cards.filter(card =>
      card.name.toLowerCase().includes(searchTerm.toLowerCase())
    )
  );

  type CostMap = {
    [key: string]: { color: string; icon: string };
  };

  const costMap: CostMap = {
    earth_land: { color: "#28b84a", icon: "leaf" }, // Earth
    dream_land: { color: "#f3ca12", icon: "cross" }, // Holy
    death_land: { color: "#af1cb6", icon: "lily" }, // Dream
    holy_land: { color: "#2a2929", icon: "skull" } // Death
  };
</script>

<main class="main">
  <h3>loaded fra databasen</h3>
  <h1>Land Types:</h1>
  <input type="text" name="search" id="" bind:value={searchTerm}>
  <div class="costs">
    {#each land_enums as c}
      <div class="cost" style="background-color: {costMap[c.name].color};">
      <span class="icon">{@html getIcon(costMap[c.name].icon)}</span>
      </div>
    {/each}
  </div>
  <h1>Cards:</h1>
  <div class="card-wrapper">
    {#each filteredCards as card}
      <Card card={card}></Card>
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