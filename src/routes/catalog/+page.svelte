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

  let showMinions: boolean = $state(false);
  let showManas: boolean = $state(false);
  let showGreen: boolean = $state(false);
  let showOrange: boolean = $state(false);
  let showRed: boolean = $state(false);
  let showPurple: boolean = $state(false);
  let showWhite: boolean = $state(false);
  let showBlack: boolean = $state(false);

  let filteredCards = $derived(
    cards.filter(card => {
      const matchesSearch = card.name.toLowerCase().includes(searchTerm.toLowerCase());
      const a = showGreen ? card.green > 0 : true;
      const b = showOrange ? card.orange > 0 : true;
      const c = showRed ? card.red > 0 : true;
      const d = showPurple ? card.purple > 0 : true;
      const e = showWhite ? card.white > 0 : true;
      const f = showBlack ? card.black > 0 : true;
      const g = showMinions ? card.type === 1 : true;
      const h = showManas ? card.type === 2 : true; 
      
      return matchesSearch && a && b && c && d && e && f && g && h;
    })
  );

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
  <div class="bar-wrapper"> 
    <input type="text" name="search" id="" placeholder="Search Catalog" bind:value={searchTerm}>
    <div class="type-switch">
      <button class="button minion-trigger" class:active={showMinions} onclick={() => { showMinions = !showMinions; if(showManas) showManas = false; } }>Minions</button>
      <button class="button mana-trigger" class:active={showManas} onclick={() => { showManas = !showManas; if(showMinions) showMinions = false } }>Manas</button>  
    </div>
    <button class="button mana green" class:active={showGreen} onclick={() => showGreen = !showGreen}><span class="icon green">{@html getIcon("manaGreen")}</span></button>
    <button class="button mana orange" class:active={showOrange} onclick={() => showOrange = !showOrange}><span class="icon orange">{@html getIcon("manaOrange")}</span></button>
    <button class="button mana red" class:active={showRed} onclick={() => showRed = !showRed}><span class="icon red">{@html getIcon("manaRed")}</span></button>
    <button class="button mana purple" class:active={showPurple} onclick={() => showPurple = !showPurple}><span class="icon purple">{@html getIcon("manaPurple")}</span></button>
    <button class="button mana white" class:active={showWhite} onclick={() => showWhite = !showWhite}><span class="icon white">{@html getIcon("manaWhite")}</span></button>
    <button class="button mana black" class:active={showBlack} onclick={() => showBlack = !showBlack}><span class="icon black">{@html getIcon("manaBlack")}</span></button>
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
  .type-switch{
    display: flex;
  }
  .minion-trigger, .mana-trigger{
    border: 1px solid $grey-mid;
    background-color: $grey-ultralight;
    color: $grey-dark;
    padding: 8px;
    &.active{
      color: $white;
      background-color: $secondary;
    }
    &:hover{
      cursor: pointer;
    }
  }
  .minion-trigger{
    border-right: none;
    border-top-left-radius: 5px;
    border-bottom-left-radius: 5px;
  }
  .mana-trigger{
    border-top-right-radius: 5px;
    border-bottom-right-radius: 5px;
  }
  .mana{
    overflow: hidden;
    border: 1px solid $grey-mid;
    border-radius: 5px;
    align-self: center;
    height: 40px;
    width: 40px;
    .icon{
      scale: 2;
      color: $white;
    }
    &:hover{
      cursor: pointer;
    }
    &.green {
      background-color: $grey-ultralight;
      .icon { color: $mana-green;}
      &.active {
        background-color: $mana-green;
        .icon { color: $white; }
      }
    }
    &.orange {
      background-color: $grey-ultralight;
      .icon { color: $mana-orange;}
      &.active {
        background-color: $mana-orange;
        .icon { color: $white; }
      }
    }
    &.red {
      background-color: $grey-ultralight;
      .icon { color: $mana-red;}
      &.active {
        background-color: $mana-red;
        .icon { color: $white; }
      }
    }
    &.purple {
      background-color: $grey-ultralight;
      .icon { color: $mana-purple; }
      &.active {
        background-color: $mana-purple;
        .icon { color: $white; }
      }
    }
    &.white {
      background-color: $grey-ultralight;
      .icon { color: $white;}
      &.active {
        background-color: $white;
        .icon { color: $mana-black; }
      }
    }

    &.black {
      background-color: $grey-ultralight;
      .icon { color: $mana-black;}
      &.active {
        background-color: $mana-black;
        .icon { color: $white; }
      }
    }
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
</style>