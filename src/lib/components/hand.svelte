<script lang="ts">

  let { hand = $bindable() } = $props();

  type Map = {
    [key: number]: string;
  };

  const oddRotationMap: Map = {
    0: "-30deg",  // Far left
    1: "-20deg",
    2: "-10deg",
    3: "-5deg",
    4: "0deg",     // Perfectly straight middle card
    5: "5deg",
    6: "10deg",
    7: "20deg",
    8: "30deg"    // Far right
  };

  // For even number of cards (2, 4, 6, 8, 10)
  const evenRotationMap: Map = {
    0: "-30deg",   // Far left
    1: "-20deg",
    2: "-12deg",
    3: "-6deg",
    4: "-3deg",    // Left center (no perfect middle)
    5: "3deg",     // Right center
    6: "6deg",
    7: "12deg",
    8: "20deg",
    9: "30deg"     // Far right
  };

  // Position maps with overlapping
  const oddPositionMap: Map = { // For 1,3,5,7,9 cards
    8: "-70px",   // Furthest left
    7: "-50px",   // Less overlap near center
    6: "-30px",
    5: "-15px",
    4: "0px",     // Center card
    3: "15px",
    2: "30px",
    1: "50px",
    0: "70px"     // Furthest right
  };

  const evenPositionMap: Map = { // For 2,4,6,8,10 cards
    9: "-75px",
    8: "-55px",
    7: "-35px",
    6: "-20px",
    5: "-10px",   // Left center pair
    4: "10px",    // Right center pair
    3: "20px",
    2: "35px",
    1: "55px",
    0: "75px"
  };

  function getMappedValues(index: number) {
    const isEven = hand.length % 2 === 0;
    const mapSize = isEven ? 10 : 9;
    const centerOffset = Math.floor((mapSize - hand.length) / 2);
    const virtualIndex = index + centerOffset;
    
    return {
      rotation: isEven ? evenRotationMap[virtualIndex] : oddRotationMap[virtualIndex],
      position: isEven ? evenPositionMap[virtualIndex] : oddPositionMap[virtualIndex]
    };
  }
</script>

<div id="hand">
  {#each hand as card, index}
    <div 
      class="cardback" 
      style="--rotation: {getMappedValues(index).rotation}; --position: {getMappedValues(index).position};"
    ></div>
  {/each}
</div>

<style lang="scss">
   #hand {
    display: flex;
    justify-content: center;
    margin: 0 auto;
    width: fit-content;
    transform-style: preserve-3d;
    perspective: 1000px;
  }
  .cardback {
    height: 140px;
    width: 100px;
    background-image: url("/media/cards/cardback.webp");
    background-size: auto 100%;
    background-position: center;
    background-repeat: no-repeat;
    border-radius: 5px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.3);
    transition: all 0.3s ease;
    transform-origin: bottom center;
    
    /* Apply rotation and position from your maps */
    left: 50%;
    transform: 
      translateX(calc(-50% + var(--position)))
      rotate(var(--rotation));
    z-index: var(--i);
    
    /* Hover effect - lifts card up and brings forward */
    &:hover {
      transform: 
       translateX(calc(-50% + var(--position)))
        rotate(var(--rotation))
        translateY(-30px);
      z-index: 100;
      box-shadow: 0 8px 16px rgba(0,0,0,0.2);
    }
  }
</style>