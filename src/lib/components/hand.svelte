<script lang="ts">
	import { scale } from "svelte/transition";
	import Card from "./card.svelte";
	import CardSmall from "./cardSmall.svelte";


  let { hand = $bindable() } = $props();

  let hoverIndex: number | null = $state(null);
  
  function setHover(index: number) {
    hoverIndex = index;
  }
  
  function clearHover() {
    hoverIndex = null;
  }
</script>

<div id="hand">
  {#each hand as card, index}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
      class="cardback" 
      style="--i: {index}; --total: {hand.length}"
      onmouseenter={() => setHover(index)}
      onmouseleave={clearHover}
    >
      <div class="default-card">
        <CardSmall {card}/>
      </div>
      
      {#if hoverIndex === index}
        <div 
          class="hover-card"
          in:scale={{ start: 0.9, duration: 250 }}
          out:scale={{ duration: 200 }}
        >
          <Card {card}/>
        </div>
      {/if}
    </div>
  {/each}
</div>

<style lang="scss">
    #hand {
    display: flex;
    justify-content: center;
    margin: 0 auto;
    width: fit-content;
    position: relative;
    height: 140px;
  }
  
  .cardback {
    position: absolute;
    height: 147px;
    width: 100px;
    transition: all 0.3s ease;
    
    /* Centered overlapping translation */
    left: 50%;
    transform: 
      translateX(calc(-50% + (var(--i) - (var(--total) - 1)/2) * 80px))
      translateY(40%);
    z-index: var(--i);
    
    &:hover {
      transform: 
        translateX(calc(-50% + (var(--i) - (var(--total) - 1)/2) * 80px))
        translateY(calc(40px + -20px));
      z-index: 100;
      box-shadow: 0 8px 16px rgba(0,0,0,0.2);
    }
    &:hover {
      .default-card {
        visibility: hidden;
      }
      .hover-card {
        display: block;
      }
    }
  }
  .hover-card {
      position: absolute;
      top: 0;
      left: -35px;
      transform: translateY(-50%); /* Lift and enlarge */
      z-index: 1000;
      display: none;
      pointer-events: none; /* Prevent hover card from blocking interactions */
    }
</style>