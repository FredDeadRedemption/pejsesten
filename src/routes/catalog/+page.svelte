<script lang="ts">  
  let { data } = $props()
  let { land_enums, cards} = $derived(data)
</script>

<h3>loaded fra databasen</h3>
<h1>Land Types:</h1>
<ul>
  {#each land_enums as le}
    <li>{le.name}</li>
  {/each}
</ul>
<h1>Cards:</h1>
<ul>
  {#each cards as card}
    <li>NAME: {card.name}</li>
    <li>ATTACK: {card.attack}</li>
    <li>DEFENCE: {card.defence}</li>
    {#if card.death_cost}
      <li>COST: {card.death_cost} death</li>
    {/if}
    {#if card.dream_cost}
      <li>COST: {card.dream_cost} dream</li>
    {/if}
    {#if card.holy_cost}
      <li>COST: {card.holy_cost} holy</li>
    {/if}
    {#if card.earth_cost}
      <li>COST: {card.earth_cost} earth</li>
    {/if}
    {#if card.race_type}
      <li>RACE: {card.race_type}</li>
    {:else}
      <li>RACE: any</li>
    {/if}
    <li>LAND TYPE: {card.land_type}</li>
    <li>DESCRIPTION: {card.description}</li>
  {/each}
</ul>