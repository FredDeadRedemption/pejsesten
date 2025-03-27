<script lang="ts">
  import type { Database } from '$lib/database.types'; 
	import { getIcon } from '$lib/icons';
	import { get } from 'svelte/store';
  type Card = Database['public']['Tables']['cards']['Row'];

  let { card } = $props<{ card: Card }>();

  type CostMap = {
    [key: string]: { color: string; icon: string };
  };

  const costMap: CostMap = {
    earth_cost: { color: "#28b84a" , icon: "leaf" }, // Earth
    holy_cost: { color: "#f3ca12", icon: "cross" }, // Holys
    dream_cost: { color: "#af1cb6", icon: "lily" }, // Dream
    death_cost: { color: "#2a2929", icon: "skull" } // Death
  };

  let primaryCostType = Object.entries({
    earth: card.earth_cost,
    dream: card.dream_cost,
    death: card.death_cost,
    holy: card.holy_cost
  }).reduce((a, b) => (a[1] > b[1] ? a : b))[0];
  let primaryCost = "standard";

  const primaryCostSYMBOL = costMap[`${primaryCostType}_cost`];


  let costs: Array<{ color: string, icon: string }> = [];

  Object.entries(costMap).forEach(([costType, costData]) => {
    const value = card[costType as keyof typeof card];
    if (typeof value === "number") {
      costs.push(...Array(value).fill(costData));
    }
  });
</script>

<div id="card" class="{primaryCost}-bg">
  <div id="content">
    <div class="title {primaryCost}">
      {card.name}
    </div>
    <div class="img-wrap">
      <img src={card.image_url} alt="" draggable="false">
    </div>
    <div class="costs {primaryCost}">
        <div class="cost" style="background-color: {primaryCostSYMBOL.color};">
         <span class="icon">{@html getIcon(primaryCostSYMBOL.icon)}</span>
        </div>
    </div>
    <div class="bottom {primaryCost}">
      {card.attack} | {card.defence}
    </div>
  </div>
</div>

<style lang="scss">
  .standard-bg{
    outline: 1px solid red;
    background-image: url("/media/cards/card-bg-standard.jpg");
  }
  .earth-bg{
    outline: 1px solid rgb(0, 220, 0);
    background-image: url("/media/cards/card-bg-earth.webp");
  }
  .holy-bg{
    outline: 1px solid rgb(255, 179, 0);
    background-image: url("/media/cards/card-bg-holy.webp");
  }
  .dream-bg{
    outline: 1px solid rgb(204, 0, 255);
    background-image: url("/media/cards/card-bg-dream.webp");
  }
  .death-bg{
    outline: 1px solid rgb(253, 252, 252);
    background-image: url("/media/cards/card-bg-death.webp");
  }
  #card{
    //scale: 0.6; // game scale
    border: 2px solid $black;
    // width: 170px; 
    // height: 250px;
    height: 147px;
    width: 100px;
    box-shadow: $box-shadow-primary;
    border-radius: 3px;
    overflow: hidden;
    user-select: none; // Prevents selection
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    padding: 2px;
    padding-bottom: 6px;
    background-size: cover;
    background-repeat: no-repeat;
    #content{
      // overflow: hidden;
      background-size: cover;
      background-repeat: no-repeat;
      width: 100%;
      height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;

      .title{
        overflow: hidden;         
        text-overflow: ellipsis; 
        white-space: nowrap;     
        text-align: left;
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
        height: 40%;
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
      }
      .costs{
        width: 100%;
        border: 2px solid $black;
        border-radius: 3px;
        display: flex;
        align-items: center;
        gap: 2px;
        width: 100%;
        //background-color: $grey-light;
        padding: 2px;
        .cost{
          color: $white;
          display: flex;
          justify-content: center;
          align-items: center;
          width: 14px;
          height: 14px;
          border-radius: 100px;
          .icon{
            display: flex;
            justify-content: center;
            align-items: center;
          }
        } 
      }
      .description{
        display: none;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        border-bottom-right-radius: 3px;
        border-bottom-left-radius: 3px;
        width: 97%;
        border-left: 2px solid $black;
        border-right: 2px solid $black;
        border-bottom: 2px solid $black;
        font-size: 0.7rem;
        flex-grow: 1;
        .text{
          padding: 5px;
        }
      }
      .bottom{  
        font-weight: 800;
        transform: translateY(8px);
        width: 100%;
        justify-self: flex-end;
        text-align: center;
        background-color: rgb(200, 186, 186);
        border: 2px solid $black;
        border-radius: 3px;
        padding:  3px;
        justify-content: space-between;
      }
    }
  }
  .holy {
  background: linear-gradient(to right, 
      rgb(255, 242, 222) 0%, 
      rgba(180, 150, 120, 0.8) 100%);
}
.holy-desc {
  background: rgba(200, 180, 150, 0.95);
}

.death {
  background: linear-gradient(to right, 
      rgb(255, 248, 248) 0%, 
      rgba(160, 160, 160, 0.8) 100%);
}
.death-desc {
  background: rgba(180, 180, 180, 0.95);
}

.dream {
  background: linear-gradient(to right, 
      rgb(241, 236, 251) 0%, 
      rgba(170, 150, 190, 0.8) 100%);
}
.dream-desc {
  background: rgba(185, 175, 190, 0.95);
}

.earth {
  background: linear-gradient(to right, 
      rgb(238, 252, 231) 0%, 
      rgba(160, 180, 150, 0.8) 100%);
}
.earth-desc {
  background: rgba(175, 190, 165, 0.95);
}
  .standard {
    background: linear-gradient(to right, 
        rgb(255, 226, 226) 0%, 
        rgba(188, 45, 45, 0.5)100%);
  }
  .standard-desc {
    background: rgba(249, 215, 215, 0.9);
  }
</style>