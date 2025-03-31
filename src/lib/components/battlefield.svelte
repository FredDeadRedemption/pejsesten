<script lang="ts">
	import CardSmall from "./cardSmall.svelte";
  import { getCardByID } from "$lib/cards";


  let { battleField = $bindable() } = $props();
</script>

{#each battleField as cardID, index}
  <div class="card-container" style="--i: {index}; --total: {battleField.length}">
    <CardSmall card={getCardByID(cardID)!}></CardSmall>
  </div>  
{/each}

<style lang="scss">
  .card-container {
    position: absolute;
    height: 147px;
    width: 100px;
    transition: all 0.3s ease;
    
    /* Centered overlapping translation */
    left: 52.4%;
    transform: 
      translateX(calc(-50% + (var(--i) - (var(--total) - 1)/2) * 110px))
  }
</style>