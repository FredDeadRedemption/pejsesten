<script lang="ts">
	import Card from '$lib/components/card.svelte';
	import { getIcon } from '$lib/icons.js';
	import html2canvas from 'html2canvas';

  let { data } = $props()
  let { land_enums, cards } = $derived(data);

  let searchTerm: string = $state("");

  let filteredCards = $derived(
    cards.filter(card =>
      card.name.toLowerCase().includes(searchTerm.toLowerCase())
    )
  );

  type CostMap = {
    [key: string]: { color: string; icon: string };
  };

  const costMap: CostMap = {
    earth_land: { color: "#28b84a", icon: "leaf" }, // Earth
    dream_land: { color: "#f3ca12", icon: "cross" }, // Holy
    death_land: { color: "#af1cb6", icon: "lily" }, // Dream
    holy_land: { color: "#2a2929", icon: "skull" } // Death
  };

  const downloadDivAsPNG = (divId: string, filename: string) => {
    const element = document.getElementById(divId);
  
    html2canvas(element ?? new HTMLElement(), {
      useCORS: true, // Attempt to load cross-origin images as CORS
      allowTaint: true, // Allow tainted canvas (but won't be readable)
    }).then(canvas => {
        // Create a download link
        const link = document.createElement('a');
        link.download = filename || 'div-image.png';
        link.href = canvas.toDataURL('image/png');
        link.click();
    });
}
</script>

<main class="main">
  <h3>loaded fra databasen</h3>
  <h1>Land Types:</h1>
  <input type="text" name="search" id="" bind:value={searchTerm}>
  <div class="costs">
    {#each land_enums as c}
      <div class="cost" style="background-color: {costMap[c.name].color};">
      <span class="icon">{@html getIcon(costMap[c.name].icon)}</span>
      </div>
    {/each}
  </div>
  <h1>Cards:</h1>
  <div class="card-wrapper">
    {#each filteredCards as card}
      <div class="x">
        <div id={card.name}>
          <Card card={card}></Card>
        </div>
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button class="download-btn" onclick={()=>downloadDivAsPNG(card.name, card.name)}>
          <span class="icon">{@html getIcon("download")}</span>
        </button>
      </div>
    {/each}
  </div>
</main>
 
<style lang="scss">
  .x{
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .download-btn{
    min-width: 100%;
    z-index: 2;
    align-self: center;
    border-radius: 5px;
    border: 1px solid red;
    height: 30px;
    width: 30px;
    background-color: $black;
    color: $primary;
    &:hover{
      cursor: pointer;
    }
  }
  .main{
    margin: 30px;
  }
  .card-wrapper{
    padding: 10px;
    background-color: $grey-mid;
    border-radius: 10px;
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
  .costs{
    width: 100%;
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
</style>