<script lang="ts">
  import type { Database } from '$lib/database.types'; 
	import { getIcon } from '$lib/icons';
	import { get } from 'svelte/store';
  type Card = Database['public']['Tables']['cards']['Row'];

  let { card } = $props<{ card: Card }>();

  const costRecord: Record<string, Record<string, string>> = {
    earth_cost: { color: "#28b84a" , icon: "skull" }, // Earth
    dream_cost: { color: "#f39c12", icon: "skull" }, // Holys
    death_cost: { color: "#3498db", icon: "skull" }, // Dream
    holy_cost: { color: "#2a2929", icon: "skull" } // Death
  };

  let costs: Array<{ color: string, icon: string }> = [];

  for (const [costType, costData] of Object.entries(costRecord)) {
    const value = card[costType as keyof Card]; 
    if (typeof value === "number") {
      costs.push(...Array(value).fill(costData));
    }
  }
</script>

<div id="card">
  <div id="content">
    <div class="title cool-mesh">
      {card.name}
    </div>
    <div class="img-wrap">
      <img src={card.image_url} alt="" draggable="false">
    </div>
    <div class="costs cool-mesh">
      {#each costs as c}
        <div class="cost" style="background-color: {c.color};">
         <div>{@html getIcon("fire")}</div>
        </div>
      {/each}
    </div>
    <div class="description cool-mesh">{card.description}</div>
    <div class="bottom cool-mesh">
      <div class="attack">{card.attack}</div>
      {#if card.race_type}
        <div class="race">{card.race_type}</div>
      {:else}
        <div class="race">any</div>
      {/if}
      <div class="defence">{card.defence}</div>
    </div>
  </div>
</div>

<style lang="scss">
  #card{
    
    //scale: 0.6; // game scale
    border: 2px solid $black;
    width: 180px;
    height: 250px;
    min-width: 160px;
    min-height: 230px;
    box-shadow: $box-shadow-primary;
    border-radius: 3px;
    overflow: hidden;
    user-select: none; // Prevents selection
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    padding: 3px;
    background-image: url("/media/cards/card-background.jpg");
    background-size: cover;
    background-repeat: no-repeat;
    #content{
      overflow: hidden;
      background-size: cover;
      background-repeat: no-repeat;
      width: 100%;
      height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;

      .title{
        width: 100%;
        border: 2px solid $black;
        border-radius: 3px;
        padding: 2px;
        font-style: oblique;
        font-size: 0.8rem;
      }
    
      .img-wrap{
        border-left: 2px solid $black;
        border-right: 2px solid $black;
        width: 97%;
        height: 100px;
        background-color: $grey-black;
        color: $white;
        display: flex;
        justify-content: center;
        align-items: center;
        overflow: hidden;
        img{
          z-index: 1;
          height: 100%;
          width: 100%;
          object-fit: cover;
        }
        .name{
          background-color: rgba(0, 0, 0, 0.5);
          padding: 2px 5px 2px 5px;
          max-width: 160px;
          text-align: center;
          display: flex;
          justify-content: center;
          align-items: center;
          position: absolute;
          z-index: 2;
        }
      }
      .costs{
        width: 100%;
        border: 2px solid $black;
        border-radius: 3px;
        display: flex;
        align-items: center;
        gap: 2px;
        width: 100%;
        background-color: $grey-light;
        padding: 2px;
        .cost{
          color: $white;
          display: flex;
          justify-content: center;
          align-items: center;
          width: 14px;
          height: 14px;
          border-radius: 100px;
        } 
      }
      .description{
        width: 97%;
        border-left: 2px solid $black;
        border-right: 2px solid $black;
        text-align: center;
        font-size: 0.7rem;
        padding: 5px;
        flex-grow: 1;
      }
      .bottom{  
        border: 2px solid $black;
        border-radius: 3px;
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        justify-content: space-between;
        width: 100%;
        .race{
          display: flex;
          align-items: center;
          justify-content: center;
          font-size: 0.8rem;
          font-style: oblique;
        }
        .attack, .defence{
          display: flex;
          justify-content: center;
          align-items: center;
        }
        .attack{
          background-color: rgba(255, 162, 0, 0.5);
        }
        .defence{
          background-color: rgba(255, 0, 0, 0.5);
        }
      }
    }
  }
  .cool-mesh{
    --s: 40px; /* control the size*/
    --c1: #fcf7f2;
    --c2: rgb(242, 242, 233);
    
    --_g: 
      #0000 calc(-650%/13) calc(50%/13),var(--c1) 0 calc(100%/13),
      #0000 0 calc(150%/13),var(--c1) 0 calc(200%/13),
      #0000 0 calc(250%/13),var(--c1) 0 calc(300%/13);
    --_g0: repeating-linear-gradient( 45deg,var(--_g));
    --_g1: repeating-linear-gradient(-45deg,var(--_g));
    background:
      var(--_g0),var(--_g0) var(--s) var(--s),
      var(--_g1),var(--_g1) var(--s) var(--s) var(--c2);
    background-size: calc(2*var(--s)) calc(2*var(--s));
  }
</style>