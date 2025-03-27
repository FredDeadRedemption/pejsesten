<script lang="ts">

  let { hand = $bindable() } = $props();

  type Map = {
    [key: number]: string;
  };

  const oddRotationMap: Map = {
    0: "-30deg", 1: "-20deg", 2: "-10deg", 3: "-5deg",
    4: "0deg", // center
    5: "5deg", 6: "10deg", 7: "20deg", 8: "30deg"   
  };

  const evenRotationMap: Map = {
    0: "-30deg", 1: "-20deg", 2: "-12deg", 3: "-6deg",
    4: "-3deg", 5: "3deg", // center pair
    6: "6deg", 7: "12deg", 8: "20deg", 9: "30deg"  
  };

  const oddPositionMap: Map = { 
    8: "-70px", 7: "-50px", 6: "-30px", 5: "-15px",
    4: "0px", // center 
    3: "15px", 2: "30px", 1: "50px", 0: "70px"     
  };

  const evenPositionMap: Map = { 
    9: "-85px", 8: "-55px", 7: "-35px", 6: "-20px",
    5: "-10px", 4: "10px", // center pair
    3: "20px", 2: "35px", 1: "55px", 0: "85px"
  };

  const oddYPositionMap: Map = {
    0: "20px", 1: "10px", 2: "4px", 3: "1px",
    4: "0px",  // center
    5: "1px", 6: "4px", 7: "10px", 8: "20px"
  };

  const evenYPositionMap: Map = {
    0: "18px", 1: "9px", 2: "4px", 3: "1px",
    4: "0px", 5: "0px", // center pair
    6: "1px", 7: "4px", 8: "9px", 9: "18px"
  };

  function getMappedValues(index: number) {
    const isEven = hand.length % 2 === 0;
    const mapSize = isEven ? 10 : 9;
    const centerOffset = Math.floor((mapSize - hand.length) / 2);
    const virtualIndex = index + centerOffset;
    
    return {
      rotation: isEven ? evenRotationMap[virtualIndex] : oddRotationMap[virtualIndex],
      position: isEven ? evenPositionMap[virtualIndex] : oddPositionMap[virtualIndex],
      yPosition: isEven ? evenYPositionMap[virtualIndex] : oddYPositionMap[virtualIndex]
    };
  }
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