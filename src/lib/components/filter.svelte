<script lang="ts">
	import { getIcon } from "$lib/icons";

  let { cards = $bindable(), filteredCards = $bindable() } = $props();
  
  let searchTerm: string = $state("");

  let showMinions: boolean = $state(false);
  let showManas: boolean = $state(false);
  let showGreen: boolean = $state(false);
  let showOrange: boolean = $state(false);
  let showRed: boolean = $state(false);
  let showPurple: boolean = $state(false);
  let showWhite: boolean = $state(false);
  let showBlack: boolean = $state(false);

  const filter = () => {
    filteredCards = cards.filter((card: any) => {
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
    });
  }
  filter();
</script>

<div class="bar-wrapper">
  <input type="text" name="search" id="" placeholder="Search Catalog" bind:value={searchTerm} onchange={filter}>
  <div class="type-switch">
    <button class="button minion-trigger" class:active={showMinions} onclick={() => { showMinions = !showMinions; if(showManas) showManas = false; filter() } }>Minions</button>
    <button class="button mana-trigger" class:active={showManas} onclick={() => { showManas = !showManas; if(showMinions) showMinions = false; filter() } }>Manas</button>  
  </div>
  <button class="button mana green" class:active={showGreen} onclick={() => {showGreen = !showGreen; filter()}}><span class="icon green">{@html getIcon("manaGreen")}</span></button>
  <button class="button mana orange" class:active={showOrange} onclick={() => {showOrange = !showOrange; filter()}}><span class="icon orange">{@html getIcon("manaOrange")}</span></button>
  <button class="button mana red" class:active={showRed} onclick={() => {showRed = !showRed; filter()}}><span class="icon red">{@html getIcon("manaRed")}</span></button>
  <button class="button mana purple" class:active={showPurple} onclick={() => {showPurple = !showPurple; filter()}}><span class="icon purple">{@html getIcon("manaPurple")}</span></button>
  <button class="button mana white" class:active={showWhite} onclick={() => {showWhite = !showWhite; filter()}}><span class="icon white">{@html getIcon("manaWhite")}</span></button>
  <button class="button mana black" class:active={showBlack} onclick={() => {showBlack = !showBlack; filter()}}><span class="icon black">{@html getIcon("manaBlack")}</span></button>
</div>

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
  .bar-wrapper{
    padding: 10px;
    background-color: $grey-light;
    border-radius: 10px;
    display: flex;
    gap: 10px;
  }
</style>