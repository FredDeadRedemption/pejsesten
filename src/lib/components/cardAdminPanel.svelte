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
<div class="pop-up-wrapper" transition:fade={{ duration: 100 }}>
  <button id="close-btn" onclick={() => {updating = !updating}}>
    <span>{@html getIcon("close")}</span>
  </button>

  <div id="update">
    
    <form method="POST" use:enhance >

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

        <label for="file-upload" class="file-upload">
          
          <img class="image" src={card.image_url} alt="" draggable="false">
          <span>{@html getIcon("new")}</span>
        
        </label>
        <input type="file" id="file-upload" name="image" accept="image/*" onchange={handleImgUpdate} />
        {console.log(card.image_url)}
        
      </div>
      <button type="submit" class="button primary">Update Card</button>
    </form>
  </div>
</div>

<div id="blur" transition:fade={{ duration: 200 }}></div>
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

  #close-btn {
    position: absolute;
    width: 2rem;
    height: 2rem;
    right: 0;
    border: none;
    background: none;
    cursor: pointer;
    color: $grey-black;
    transition: 250ms ease all;
    &:hover{
      color: $grey-dark;
    }
  }

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
    width: 20rem;
  }

  #update{
    border-radius: 6px;
    background-color: $grey-light;
    padding: 15px;
    display: flex;
    flex-direction: column;
    gap: 15px;
    button{
      width: 100%;
    }
  }
  input, textarea, .file-upload{
    text-align: center;
    background-color: $grey-mid;
    border: none;
    padding: 10px;
    border-radius: 4px;
    &:focus{
      outline: none;
    }
  }
  textarea{
    resize: none;
    height: 70px;
  }
  .grp{
    display: flex;
    flex-direction: column;
  }
  
  .grp-side-by-side-four{
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    gap: 15px;
    input{
      width: 100%;
    }
  }
  label{
    align-self: center;
  }
  input[type="number"]::-webkit-inner-spin-button,
  input[type="number"]::-webkit-outer-spin-button {
    -webkit-appearance: none; /* WebKit browsers */
    margin: 0; /* Optional: Remove margin */
  }

  input[type="file"] {
    display: none;
  }

  .file-upload {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: space-evenly;
    align-items: center;
    cursor: pointer;
    color: $grey-black;
    transition: 250ms ease all;
    &:hover{
      color: $grey-dark;
    }
  }
  .image {
    z-index: 1;
    height: 50%;
    width: 50%;
    object-fit: cover;
    border: none;
    border-radius: 3px;
    filter: drop-shadow(2px 2px 2px $grey-dark);
    
  }
  
</style>
