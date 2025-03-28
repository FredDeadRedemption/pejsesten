<script lang="ts">
	import { scale, fly } from "svelte/transition";
  import type { Database } from '$lib/database.types'; 
	import Card from "./card.svelte";
	import CardSmall from "./cardSmall.svelte";
  type CardT = Database['public']['Tables']['cards']['Row'];


  let { hand = $bindable(), onPlaceCard } = $props();

  let hoverIndex: number | null = $state(null); // keeps track of which index to display big car
  let draggerIndex: number | null = $state(null); // keeps track of which index is being dragged
  let dragCoords = $state({ x: 0, y: 0});
  let dragCard: CardT | null = $state(null);

  const setHover = (index: number) =>hoverIndex = index;
  const clearHover = () => hoverIndex = null;

  let draggin: boolean = $state(false);
  $effect(()=>{console.log(draggin)})

  const beginDrag = (index: number, event: MouseEvent) => {
    event.preventDefault();
    console.log("draggin")
    draggin = true;
    draggerIndex = index;
    dragCoords = {
      x: event.clientX -50,
      y: event.clientY -73
    };
    dragCard = hand[index];
  }
  const endDrag = () => {
    console.log("chilling")
    draggerIndex = null;
    draggin = false;
    if (!onPlaceCard) return;
    onPlaceCard(dragCoords.x, dragCoords.y, dragCard)
    dragCard = null;
  }
  const onMouseMove = (e: { movementX: number; movementY: number; }) => {
		if (!draggin) return;
		dragCoords.x += e.movementX;
	  dragCoords.y += e.movementY;
	};
</script>

<svelte:window onmouseup={endDrag} onmousemove={onMouseMove}></svelte:window>

<div id="hand">
  {#each hand as card, index}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
      class="card-container" 
      style="--i: {index}; --total: {hand.length}"
      onmouseenter={() => setHover(index)}
      onmouseleave={clearHover}
    >

      {#if hoverIndex === index && !draggin}
        <div 
          class="hover-card"
          onmousedown={(e: MouseEvent)=>beginDrag(index, e)}
          in:scale={{ start: 0.9, duration: 250 }}
          out:scale={{ duration: 200 }}
        >
          <Card {card}/>
        </div>
      {:else if draggerIndex !== index}
        <div class="default-card">
            <CardSmall {card}/>
        </div>
      {/if}
    </div>
  {/each}
  {#if draggin && dragCard}
      <div class="dragger" style="position: abosolute; left: {dragCoords.x}px; top: {dragCoords.y}px;"
      >
        <CardSmall card={dragCard}></CardSmall>
      </div>
  {/if}
</div>

<style lang="scss">
  .dragger{
    position: fixed; /* Use fixed for smooth dragging */
    z-index: 1000;
    pointer-events: none;
    cursor: grabbing;
  }

  #hand {
    display: flex;
    justify-content: center;
    margin: 0 auto;
    width: fit-content;
    position: relative;
    height: 100%;
  }
  
  .card-container {
    position: absolute;
    height: 147px;
    width: 100px;
    transition: all 0.3s ease;
    
    /* Centered overlapping translation */
    left: 50%;
    transform: 
      translateX(calc(-50% + (var(--i) - (var(--total) - 1)/2) * 80px))
      translateY(10%);
  }
  .hover-card {
      cursor: pointer;
      border: 2px solid greenyellow;
      border-radius: 5px;
      position: absolute;
      top: 0;
      left: -35px;
      transform: translateY(-60%);
    }
</style>