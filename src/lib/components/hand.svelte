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
    4: "0deg",    // Left center (no perfect middle)
    5: "0deg",     // Right center
    6: "6deg",
    7: "12deg",
    8: "20deg",
    9: "30deg"     // Far right
  };

  // Corresponding position maps
  const positionMap: Map = {
    0: "-80px",
    1: "-60px",
    2: "-40px",
    3: "-20px",
    4: "-10px",
    5: "10px",
    6: "20px",
    7: "40px",
    8: "60px",
    9: "80px"
  };

  function getMappedValues(index: number) {
    const isEven = hand.length % 2 === 0;
    const mapSize = isEven ? 10 : 9;
    const centerOffset = Math.floor((mapSize - hand.length) / 2);
    const virtualIndex = index + centerOffset;
    
    return {
      rotation: isEven ? evenRotationMap[virtualIndex] : oddRotationMap[virtualIndex],
      position: positionMap[virtualIndex]
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
    
      rotate(var(--rotation));
    z-index: var(--i);
    
    /* Hover effect - lifts card up and brings forward */
    &:hover {
      transform: 
        rotate(var(--rotation))
        translateY(-30px);
      z-index: 100;
      box-shadow: 0 8px 16px rgba(0,0,0,0.2);
    }
  }
</style>