<script lang="ts">
  import { getIcon } from '$lib/icons.js';
  import { downloadDivAsPNG } from '$lib/util';
  import type { SupabaseClient } from '@supabase/supabase-js';
  import type { Database } from '$lib/database.types'; 
	import { fade, slide } from 'svelte/transition';
	import { enhance } from '$app/forms';
	import { fail } from '@sveltejs/kit';


  type Card = Database['public']['Tables']['cards']['Row'];


  let { card, supabase, onDeleteCard, onUpdateCard }: { card: Card, supabase: SupabaseClient, onDeleteCard: any, onUpdateCard: any} = $props()

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

  let cardToUpdate: Card | null = $state(null);
  let updating: boolean = $state(false);

  const setUpdatingCard = (card : Card) =>{
    cardToUpdate = JSON.parse(JSON.stringify(card));
    newImageBase64 = cardToUpdate!.image_url;
  }
  
  let newImageFile : File | null = $state(null);
  let newImageBase64 : string | null = $state(null);


  const handleImgUpdate = (event : Event) => {

    const target = event.target as HTMLInputElement;

    const img = target.files?.[0];
    

    if (img) {
      newImageFile = img;
      console.log(newImageFile);

      const reader = new FileReader();

      reader.onload = (e) => {
        if(cardToUpdate) {
          newImageBase64 = e.target?.result as string;
        }
      }
      reader.readAsDataURL(img);

    }
  }

  const updateCard = async() => {
    if(!cardToUpdate) return;
    let card = cardToUpdate;

    updating = false;    

    console.log(card.image_url)

    try {
      // First, get the existing card to check if it exists and preserve values that aren't being updated
      const { data: existingCard, error: fetchError } = await supabase
        .from('cards')
        .select('*')
        .eq('id', card.id)
        .single();
      
      if (fetchError || !existingCard) {
        return fail(404, { error: 'Card not found' });
      }

      if (newImageFile) {

        console.log("updateCard")

        //-----Delete old image-----
        const fullPath = existingCard.image_url; 
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

        //-----Upload new image-----
        if (newImageFile && newImageFile.size > 0) {
          const fileExt = newImageFile.name.split('.').pop();
          const fileName = `card-${card.name.replaceAll(" ", "-")}${new Date().getTime()}.${fileExt}`;
          const filePath = `cards/${fileName}`; // Store in a "cards" folder for organization

          const { data: uploadData, error} = await supabase.storage
            .from('card-images') // Your bucket name
            .upload(filePath, newImageFile, {
              cacheControl: '3600', // Cache for 1 hour
              upsert: true // Do not overwrite existing files
            });

          if (error) {
            console.error('Upload error:', JSON.stringify(error, null, 2));
            throw error;
          }

          // Get the public URL of the uploaded image
          const { data: { publicUrl } } = supabase.storage
            .from('card-images')
            .getPublicUrl(uploadData.path);

          card.image_url = publicUrl;
        }
      }

      
      const { data: updatedCard, error: updateError } = await supabase
        .from('cards')
        .update({
          name: card.name,
          attack: card.attack,
          defence: card.defence,
          white: card.white,
          black: card.black,
          purple: card.purple,
          green: card.green,
          red: card.red,
          orange: card.orange,
          description: card.description,
          race: card.race,
          image_url: card.image_url
        })
        .eq('id', card.id)
        .select(); 
        
      if (updateError) {
        console.error("Update error: " + updateError);
        throw updateError;
      }

      card.image_url = newImageBase64!;
      newImageBase64 = null;
      newImageFile = null;
      onUpdateCard(card);
      return { success: true, card: updatedCard[0] };
    } catch (error) {
      console.error('Error updating card:', error);
      return fail(500, { error: 'Failed to update card. Please try again.' });
    }
  }
</script>

<div id="panel">
  {#if !deleting}
    <button transition:slide={{ axis: "x", duration: 100 }} class="btn download" onclick={() => downloadDivAsPNG(card.name, card.name)}>
      <span class="icon">{@html getIcon("png")}</span>
    </button>
    <button transition:slide={{ axis: "x", duration: 100 }} class="btn edit" onclick={() => {updating= !updating; setUpdatingCard(card)}}>
      <span class="icon">{@html getIcon("update")}</span>
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
    

      <div class="grp">
        <label for="name">Name</label>
        <input type="text" id="name" name="name" required bind:value={cardToUpdate!.name}/>
      </div>

      <div class="grp-side-by-side-four">
        <div class="grp">
          <label for="holyCost">Holy</label>
          <input type="number" id="holyCost" name="holyCost" bind:value={cardToUpdate!.white}/>
        </div>

        <div class="grp">
          <label for="deathCost">Death</label>
          <input type="number" id="deathCost" name="deathCost" bind:value={cardToUpdate!.black}/>
        </div>

        <div class="grp">
          <label for="dreamCost">Dream</label>
          <input type="number" id="dreamCost" name="dreamCost" bind:value={cardToUpdate!.purple}/>
        </div>

        <div class="grp">
          <label for="earthCost">Earth</label>
          <input type="number" id="earthCost" name="earthCost" bind:value={cardToUpdate!.green}/>
        </div>

        <div class="grp">
          <label for="fireCost">Red</label>
          <input type="number" id="fireCost" name="fireCost" bind:value={cardToUpdate!.red}/>
        </div>

        <div class="grp">
          <label for="orangeCost">Orange</label>
          <input type="number" id="orangeCost" name="orangeCost" bind:value={cardToUpdate!.orange}/>
        </div>
      </div>

      <div class="grp">
        <label for="description">Description</label>
        <textarea id="description" name="description" bind:value={cardToUpdate!.description}></textarea>
      </div>

      <div class="grp">
        <label for="image">Image</label>

        <label for="file-upload" class="file-upload">
          
          <img class="image" src={newImageBase64} alt="" draggable="false">
          <span>{@html getIcon("new")}</span>
        
        </label>
        <input type="file" id="file-upload" name="image" accept="image/*" onchange={handleImgUpdate} />
        
      </div>
      <button onclick={updateCard} class="button primary">Update Card</button>
  </div>
</div>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div id="blur" onclick={() => {updating = false}} transition:fade={{ duration: 200 }}></div>
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
    z-index: 99;
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
    z-index: 100;
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
