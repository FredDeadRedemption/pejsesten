<script lang="ts">
  import { getIcon } from '$lib/icons.js';
  import { downloadDivAsPNG } from '$lib/util';
  import type { SupabaseClient } from '@supabase/supabase-js';
  import type { Database } from '$lib/database.types'; 
	import { slide } from 'svelte/transition';
  type Card = Database['public']['Tables']['cards']['Row'];

  let { card, supabase, onDeleteCard }: { card: Card, supabase: SupabaseClient, onDeleteCard: any} = $props()

  const deleteCard = async (id: number) => {
    if(!id) return;

    const { data: cardResponse, error: fetchError } = await supabase
      .from("cards")
      .select("image_url")
      .eq("id", id)
      .single();

    if (fetchError) {
      console.error("Error fetching card:", fetchError);
      return;
    }

    const fullPath = cardResponse.image_url; 
    const filename = fullPath.split('/').pop(); 
    console.log("Filename:", filename); 

    const { error: deleteImageError } = await supabase
      .storage
      .from("card-images")
      .remove([`cards/${filename}`]);

    if (deleteImageError) {
      console.error("Error deleting image:", deleteImageError);
      return;
    }

    // Delete the card
    const { error: deleteCardError } = await supabase
      .from("cards")
      .delete()
      .eq("id", id);

    if (deleteCardError) {
      console.error("Error deleting card:", deleteCardError);
    }
    onDeleteCard(id);
  }

  let deleting: boolean = $state(false);

  const beginDelete = () => {
    deleting = true;
  }
</script>

<div id="panel">
  {#if !deleting}
    <button transition:slide={{ axis: "x", duration: 100 }} class="btn download" onclick={() => downloadDivAsPNG(card.name, card.name)}>
      <span class="icon">{@html getIcon("png")}</span>
    </button>
    <button transition:slide={{ axis: "x", duration: 100 }} class="btn edit" onclick={() => downloadDivAsPNG(card.name, card.name)}>
      <span class="icon">{@html getIcon("creator")}</span>
    </button>
  {/if}
  <button class="btn delete" onmouseleave={() => deleting = false} onclick={() => deleting ? deleteCard(card.id) : beginDelete()}>
    <span class="icon">{@html getIcon("delete")}</span>
  </button>
</div>


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
</style>
