<script lang="ts">
  import type { Database } from '$lib/database.types'; 
  type Card = Database['public']['Tables']['cards']['Row'];

  let { card } = $props<{ card: Card }>();

  const costColors: Record<string, string> = {
    earth_cost: "#28b84a", // Earth
    dream_cost: "#f39c12", // Dream
    death_cost: "#3498db", // Death
    holy_cost: "#e74c3c", // Holy
  };

  let costs: string[] = [];
  for (const [costType, color] of Object.entries(costColors)) {
    const value = card[costType as keyof Card]; 
    if (typeof value === "number") {
      costs.push(...Array(value).fill(color));
    }
  }
</script>

<div id="card">
  <div class="img-name-wrap">
    <img src={card.image_url} alt="" draggable="false">
    <span class="name">{card.name}</span>
  </div>
  <div class="costs">
    {#each costs as c}
      <div class="cost" style="background-color: {c};"></div>
    {/each}
  </div>
  <div class="description">{card.description}</div>
  <div class="bottom">
    <div class="attack">{card.attack}</div>
    {#if card.race_type}
      <div class="race">{card.race_type}</div>
    {:else}
      <div class="race">any</div>
    {/if}
    <div class="defence">{card.defence}</div>
  </div>
</div>

<style lang="scss">
  #card{
    //scale: 0.6; // game scale
    display: flex;
    flex-direction: column;
    width: 160px;
    height: 230px;
    min-width: 160px;
    min-height: 230px;
    box-shadow: $box-shadow-primary;
    border-radius: 5px;
    overflow: hidden;
    user-select: none; // Prevents selection
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    .img-name-wrap{
      width: 100%;
      height: 80px;
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
      display: flex;
      align-items: center;
      gap: 3px;
      width: 100%;
      height: 18px;
      background-color: $grey-light;
      padding-left: 3px;
      .cost{
        width: 12px;
        height: 12px;
        border-radius: 100px;
      } 
    }
    .description{
      text-align: center;
      background-color: $grey-mid;
      font-size: 0.8rem;
      padding: 5px;
      flex-grow: 1;
    }
    .bottom{  
      display: flex;
      justify-content: space-between;
      background-color: $grey-light;
      width: 100%;
      .race{
        display: flex;
        align-items: center;
        font-size: 0.8rem;
      }
      .attack, .defence{
        display: flex;
        justify-content: center;
        align-items: center;
        height: 18px;
        width: 18px;
      }
      .attack{
        background-color: gold;
      }
      .defence{
        background-color: $primary;
      }
    }
  }
</style>