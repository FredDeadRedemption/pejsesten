<script lang="ts">
	import Card from '$lib/components/card.svelte';
	import CardAdminPanel from '$lib/components/cardAdminPanel.svelte';
	import { getIcon } from '$lib/icons.js';
  import type { Database } from '$lib/database.types'; 
  type CardT = Database['public']['Tables']['cards']['Row'];

  let { data } = $props()
  let { profile, supabase } = $derived(data);
  let { cards } = $state(data);

  let searchTerm: string = $state("");

  let showGreen: boolean = $state(false);
  let showOrange: boolean = $state(false);
  let showRed: boolean = $state(false);
  let showBlue: boolean = $state(false);
  let showWhite: boolean = $state(false);
  let showBlack: boolean = $state(false);

  let filteredCards = $derived(
    cards.filter(card => {
      const matchesSearch = card.name.toLowerCase().includes(searchTerm.toLowerCase());
      const a = showGreen ? card.green > 0 : true;
      const b = showOrange ? card.orange > 0 : true;
      const c = showRed ? card.red > 0 : true;
      const d = showBlue ? card.purple > 0 : true;
      const e = showWhite ? card.white > 0 : true;
      const f = showBlack ? card.black > 0 : true;
      
      return matchesSearch && a && b && c && d && e && f;
    })
  );

  const onDeleteCard = (id: number) => cards = cards.filter(card => card.id != id);
  const onUpdateCard = (updatedCard: CardT) => {
    let i = cards.findIndex(card => card.id === updatedCard.id);
    cards[i] = updatedCard;
  }
 
  type CostMap = {
    [key: string]: { color: string; icon: string };
  };

  const costMap: CostMap = {
    earth_land: { color: "#28b84a", icon: "manaGreen" }, // Earth
    holy_land: { color: "#f3ca12", icon: "manaWhite" }, // Holy
    dream_land: { color: "#af1cb6", icon: "manaPurple" }, // Dream
    death_land: { color: "#2a2929", icon: "manaBlack" } // Death
  };
</script>

<main class="main">
  <div class="bar-wrapper"> 
    <input type="text" name="search" id="" placeholder="Search Catalog" bind:value={searchTerm}>
    <button class="button" class:active={showGreen} onclick={() => showGreen = !showGreen}>GREEN</button>
    <button class="button" class:active={showOrange} onclick={() => showOrange = !showOrange}>ORANGE</button>
    <button class="button" class:active={showRed} onclick={() => showRed = !showRed}>RED</button>
    <button class="button" class:active={showBlue} onclick={() => showBlue = !showBlue}>BLUE</button>
    <button class="button" class:active={showWhite} onclick={() => showWhite = !showWhite}>WHITE</button>
    <button class="button" class:active={showBlack} onclick={() => showBlack = !showBlack}>BLACK</button>
  </div>
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
  .active{
    background-color: $ok;
  }
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