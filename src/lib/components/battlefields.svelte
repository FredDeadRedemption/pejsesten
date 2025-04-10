<script lang="ts">
	import CardSmall from "./cardSmall.svelte";
  import { getCardByID } from "$lib/cards";
  import { attack } from "$lib/socket/socket";

  let { selfBattleField = $bindable(), enemyBattleField = $bindable() } = $props();

  let attacking: boolean = $state(false);
  let origin: number | null = $state(null);

  const beginAttack = (index: number) => {
    console.log("ATTACKING WITH INDEX: ", index)
    if(attacking) return;
    attacking = true;
    origin = index;
  }

  const tryAttack = (index: number) => {
    if(!attacking) return;
    if(origin === null) return;
    console.log("TRYING TO ATTACK INDEX: ", index)
    attack({
      origin: origin,
      target: index,
      face: false, // TODO: man skal kunne attacke face
    });
    attacking = false;
    origin = null;
  }

  const cancelAttack = () => {
    console.log("CANCEL ATTACK")
    attacking = false;
    origin = null;
  }
</script>

<svelte:window onclick={cancelAttack}></svelte:window>


<div class="enemy-battlefield">
  {#each enemyBattleField as card, index}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="card-container" style="--i: {index}; --total: {enemyBattleField.length}" onclick={(e) => {
      e.stopPropagation() // so it doesnt also trigger cancelAttack prevent event bubbling
      tryAttack(index);
    }}>
      <CardSmall card={card!}></CardSmall>
    </div>  
  {/each}
</div>
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="self-battlefield" onclick={cancelAttack}>
  {#each selfBattleField as card, index}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="card-container" style="--i: {index}; --total: {selfBattleField.length}" onclick={(e) => {
      e.stopPropagation() // so it doesnt also trigger cancelAttack prevent event bubbling
      beginAttack(index);
    }}>
      <CardSmall card={card!}></CardSmall>
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