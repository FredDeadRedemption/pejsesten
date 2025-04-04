<script lang="ts">
	import CardSmall from "./cardSmall.svelte";
  import { getCardByID } from "$lib/cards";


  let { selfBattleField = $bindable(), enemyBattleField = $bindable() } = $props();
</script>

<div class="enemy-battlefield">
  {#each enemyBattleField as cardID, index}
    <div class="card-container" style="--i: {index}; --total: {enemyBattleField.length}">
      <CardSmall card={getCardByID(cardID)!}></CardSmall>
    </div>  
  {/each}
</div>
<div class="self-battlefield">
  {#each selfBattleField as cardID, index}
    <div class="card-container" style="--i: {index}; --total: {selfBattleField.length}">
      <CardSmall card={getCardByID(cardID)!}></CardSmall>
    </div>  
  {/each}
</div>

<style lang="scss">
  .self-battlefield, .enemy-battlefield{
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .card-container {
    position: absolute;
    height: 147px;
    width: 100px;
    transition: all 0.3s ease;
    
    /* Centered overlapping translation */
    left: 50%;
    transform: 
      translateX(calc(-50% + (var(--i) - (var(--total) - 1)/2) * 110px))
  }
</style>