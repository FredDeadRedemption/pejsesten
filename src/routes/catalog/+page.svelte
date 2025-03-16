<script lang="ts">
	import Card from '$lib/components/card.svelte';

  
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
<div class="card-wrapper">
  {#each cards as card}
    <Card card={card}></Card>
  {/each}
</div>
 
<style lang="scss">
  .card-wrapper{
    padding: 10px;
    background-color: $grey-mid;
    border-radius: 10px;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
</style>