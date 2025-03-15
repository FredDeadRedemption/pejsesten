<script lang="ts">
  import type { Database } from '$lib/database.types'; 
  type Card = Database['public']['Tables']['cards']['Row'];

  let { card } = $props<{ card: Card }>();
</script>

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
<li>DESCRIPTION: {card.description}</li>