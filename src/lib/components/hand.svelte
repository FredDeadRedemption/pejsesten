<script lang="ts">

  let { hand = $bindable() } = $props();
</script>

<div id="hand">
  {#each hand as card, index}
    <div 
      class="cardback" 
      style="--i: {index}; --total: {hand.length}"
    ></div>
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
    height: 140px;
    width: 100px;
    background-image: url("/media/cards/cardback.webp");
    background-size: auto 100%;
    background-position: center;
    background-repeat: no-repeat;
    border-radius: 5px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.3);
    transition: all 0.3s ease;
    
    /* Centered overlapping translation */
    left: 50%;
    transform: 
      translateX(calc(-50% + (var(--i) - (var(--total) - 1)/2) * 60px));
    z-index: var(--i);
    
    &:hover {
      transform: 
        translateX(calc(-50% + (var(--i) - (var(--total) - 1)/2) * 60px))
        translateY(-20px);
      z-index: 100;
      box-shadow: 0 8px 16px rgba(0,0,0,0.2);
    }
  }
</style>