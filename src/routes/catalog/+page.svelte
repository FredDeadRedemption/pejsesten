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
    <li>CARD NAME: {card.name}</li>
    <li>BASE DAMAGE: {card.base_damage}</li>
    <li>BASE DEFENCE: {card.base_defence}</li>
    <li>LAND TYPE: {card.land_type}</li>
  {/each}
</ul>