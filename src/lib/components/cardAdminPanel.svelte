<script lang="ts">
  import { getIcon } from '$lib/icons.js';
  import { downloadDivAsPNG } from '$lib/util';
  import type { SupabaseClient } from '@supabase/supabase-js';
  import type { Database } from '$lib/database.types'; 
	import { fade, slide } from 'svelte/transition';
	import { enhance } from '$app/forms';
  type Card = Database['public']['Tables']['cards']['Row'];

  let { card, supabase, onDeleteCard }: { card: Card, supabase: SupabaseClient, onDeleteCard: any} = $props()

  const deleteCard = async (id: number) => {
    if(!id) return;

    // Get card image url for later :p
    const { data: cardResponse, error: fetchError } = await supabase
      .from("cards")
      .select("image_url")
      .eq("id", id)
      .single();

    if (fetchError) {
      console.error("Error fetching card:", fetchError);
      return;
    }

    // Construct filepath
    const fullPath = cardResponse.image_url; 
    const filename = fullPath.split('/').pop(); 
    console.log("Filename:", filename); 

    // Delete the card image with the filepath
    const { error: deleteImageError } = await supabase
      .storage
      .from("card-images")
      .remove([`cards/${filename}`]);

    if (deleteImageError) {
      console.error("Error deleting image:", deleteImageError);
      return;
    }

    // Delete the card itself
    const { error: deleteCardError } = await supabase
      .from("cards")
      .delete()
      .eq("id", id);

    if (deleteCardError) {
      console.error("Error deleting card:", deleteCardError);
    }

    // remove it from the ui - voila
    onDeleteCard(id);
  }

  let deleting: boolean = $state(false);

  const beginDelete = () => deleting = true;

  let updating: boolean = $state(false);


  const handleImgUpdate = (event : Event) => {
    const target = event.target as HTMLInputElement;

    const img = target.files?.[0];

    if (img) {
      const reader = new FileReader();

      reader.onload = (e) => {
        if(card) {
          card.image_url = e.target?.result as string;
          console.log(card.image_url)
        }
      }
      reader.readAsDataURL(img);

      // console.log(img);
    }
  } 
</script>

<div id="panel">
  {#if !deleting}
    <button transition:slide={{ axis: "x", duration: 100 }} class="btn download" onclick={() => downloadDivAsPNG(card.name, card.name)}>
      <span class="icon">{@html getIcon("png")}</span>
    </button>
    <button transition:slide={{ axis: "x", duration: 100 }} class="btn edit" onclick={() => {updating= !updating}}>
      <span class="icon">{@html getIcon("creator")}</span>
    </button>
  {/if}
  <button class="btn delete" onmouseleave={() => deleting = false} onclick={() => deleting ? deleteCard(card.id) : beginDelete()}>
    <span class="icon">{@html getIcon("delete")}</span>
  </button>
</div>


{#if updating}
<div class="pop-up-wrapper">
  <button onclick={() => {updating = !updating}}>X</button>
  <form  method="POST" action="?/updateCard" use:enhance enctype="multipart/form-data">
    <div class="grp">
      <label for="name">Name</label>
      <input type="text" id="name" name="name" required bind:value={card.name}/>
    </div>

    <div class="grp-side-by-side-four">
      <div class="grp">
        <label for="holyCost">Holy</label>
        <input type="number" id="holyCost" name="holyCost" bind:value={card.holy_cost}/>
      </div>

      <div class="grp">
        <label for="deathCost">Death</label>
        <input type="number" id="deathCost" name="deathCost" bind:value={card.death_cost}/>
      </div>

      <div class="grp">
        <label for="dreamCost">Dream</label>
        <input type="number" id="dreamCost" name="dreamCost" bind:value={card.dream_cost}/>
      </div>

      <div class="grp">
        <label for="earthCost">Earth</label>
        <input type="number" id="earthCost" name="earthCost" bind:value={card.earth_cost}/>
      </div>
    </div>

    <div class="grp">
      <label for="description">Description</label>
      <textarea id="description" name="description" bind:value={card.description}></textarea>
    </div>

    <div class="grp">
      <label for="image">Image</label>
      <input type="file" id="image" name="image" accept="image/*" onchange={handleImgUpdate} />
      {console.log(card.image_url)}
      <img src={card.image_url} alt="" draggable="false">
    </div>
    <button type="submit" class="button primary">Update Card</button>
  </form>
</div>

<div id="blur" transition:fade={{ duration: 100 }}></div>
{/if}


<style lang="scss">
  #panel{
    display: flex;
    flex-direction: row;
    justify-content: space-evenly;
    overflow: hidden;
    background-color: $grey-ultralight;
    border-radius: 5px;
    border: 1px solid $grey-dark;
    color: $black;
    min-width: 100%; 
    z-index: 2;
  }
  .icon{
    scale: 0.8;
  }
  .btn{
    width: 100%;
    border: none;
    background-color: inherit;
    align-self: center;
    height: 30px;
    transition: ease all 100ms;

    &:hover{
      cursor: pointer;
      background-color: red;
    }
  }
  .download:hover { background-color: $blue; color: $white; }
  .delete:hover { background-color: $red; color: $white; }
  .edit:hover { background-color: $yellow; color: $white; }


  #blur{
    z-index: 50;
    position: fixed;
    height: 100vh;
    width: 100vw;
    top: 0px;
    left: 0px;
    background-color: rgba($white, 0.45);
    backdrop-filter: blur(3px);
    pointer-events: all;
  }
  .pop-up-wrapper {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 9999999;
    background-color: wheat;
  }
  
</style>
