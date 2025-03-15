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
<ul>
  {#each cards as card}
    <Card card={card}></Card>
  {/each}
</ul>