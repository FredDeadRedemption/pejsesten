<script lang="ts">
  import { enhance } from '$app/forms';
	import { getIcon } from '$lib/icons.js';
  import type { Database } from '$lib/database.types'; 
	import Card from '$lib/components/card.svelte';
  type CardT = Database['public']['Tables']['cards']['Row'];

  let { data } = $props()
  let { profile} = $derived(data)

  type creatingT = "minion" | "mana" | "spell" | "incantation";

  let creating: creatingT = $state("minion");
  let cardImage : string | null = $state(null);

  type mana = {
     color: string; icon: string, name: string
  };

  const manas: mana[] = [
    { color: " #38761d", icon: "manaGreen", name: "green" }, 
    { color: "white", icon: "manaWhite", name: "white" }, 
    { color: "#474ea7", icon: "manaPurple", name: "purple" }, 
    { color: "#434343", icon: "manaBlack", name: "black" }, 
    { color: "#cc0000", icon: "manaRed", name: "red" },
    { color: "#bc874f", icon: "manaOrange", name: "orange" }
  ];

  type x = {
    [key: string]: number
  }

  let manaCounts: x = $state({
    green: 0,
    orange: 0,
    red: 0,
    purple: 0,
    white: 0,
    black: 0,
  });

  let name = $state("");
  let attack = $state(null);
  let defence = $state(null);
  let description = $state("");

  let testCard = $derived<CardT>({
	  id: 0,
	  name: name,
	  attack: attack,
	  defence: defence,
	  description: description,
	  green: manaCounts.green,
	  orange: manaCounts.orange,
	  red: manaCounts.red,
	  purple: manaCounts.purple,
	  white: manaCounts.white,
	  black: manaCounts.black,
	  image_url: cardImage ?? "",
	  type: creating === "minion" ? 1 : creating === "mana" ? 2 : creating === "spell" ? 3 : 4,
	  neutral: 0,
	  race: null
  })

  const handleImgUpdate = (event : Event) => {
    const target = event.target as HTMLInputElement;
    const img = target.files?.[0];

    if (img) {
      const reader = new FileReader();

      reader.onload = (e) => {
        cardImage = e.target?.result as string;
      }
      reader.readAsDataURL(img);
  }
}

</script>

<main class="main">
  <!-- CREATE FORM -->
  <div class="create-wrap">
    <div class="form-selector">
      <button class="button" aria-label="minion" class:active={creating === "minion"} onclick={()=> { creating = "minion"; cardImage = null;}}>Create Minion</button>
      <button class="button" aria-label="mana" class:active={creating === "mana"} onclick={()=> { creating = "mana"; cardImage = null;}}>Create Mana</button>
      <button class="button" aria-label="spell" class:active={creating === "spell"} onclick={()=> { creating = "spell"; cardImage = null;}}>Create Spell</button>
      <button class="button" aria-label="incantation" class:active={creating === "incantation"} onclick={()=> { creating = "incantation"; cardImage = null;}}>Create Incantation</button>
    </div>
    {#if profile?.is_admin}
      <form id="create" method="POST" action="?/createCard" use:enhance enctype="multipart/form-data" onsubmit={()=>(cardImage=null)}>
        {#if creating === "minion"}
          <input type="hidden" id="type" name="type" value="1" required>
        {:else if creating === "mana"}
          <input type="hidden" id="type" name="type" value="2" required>
        {:else if creating === "spell"}
          <input type="hidden" id="type" name="type" value="3" required>
        {:else if creating === "incantation"}
          <input type="hidden" id="type" name="type" value="4" required>
        {/if}

        <div class="name-attack-defence" class:not-minion={creating !== "minion"}>
          <input bind:value={name} type="text" id="name" name="name" placeholder="Name" required />
          {#if creating === "minion"}
            <div class="attack-defence">
                <input bind:value={attack} type="number" id="attack" name="attack" placeholder="Attack" required />
                <input bind:value={defence} type="number" id="defence" name="defence" placeholder="Defence" required />
            </div>
          {/if}
        </div>

        <div class="manas-description">
            {#each manas as mana}
              <div class="mana">
                <div class="mana-switch">
                  <button type="button" class="add" onclick={() => { 
                    if(manaCounts[mana.name] < 3){
                       manaCounts[mana.name]++ 
                      }
                  }}>+</button>
                  <div class="icon-wrap">
                    <span class="icon" style="color: {mana.color};">{@html getIcon(mana.icon)}</span>
                  </div>
                  <button type="button" class="sub" onclick={() => { 
                    if(manaCounts[mana.name] > 0){
                       manaCounts[mana.name]-- 
                      }
                  }}>-</button>
                </div>
                <input bind:value={manaCounts[mana.name]} defaultValue="0" type="hidden" id={mana.name} name={mana.name} />
              </div>
            {/each} 
            <textarea bind:value={description} id="description" name="description" placeholder="Description"></textarea>
        </div>

        <label for="file-upload" class="file-upload">
  
          {#if cardImage}
            <img class="image" src={cardImage} alt="" draggable="false">
          {/if}
          <span>{@html getIcon("new")}</span>
        
        </label>
        <input type="file" id="file-upload" name="image" accept="image/*" onchange={handleImgUpdate} />   
        <button type="submit" class="button primary">Create Card</button>
      </form>
    {:else}
      <h1>Only admin users can use this page :(</h1>
    {/if}
  </div>
  <div class="showcase">
    <Card card={testCard} />
  </div>
</main>


<style lang="scss">
  .main{
    align-self: center;
    margin: 30px;
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 10px;
    .create-wrap{
      display: flex;
      flex-direction: column;
      gap: 10px;
    }
    .showcase{
      background-color: $grey-light;
      border-radius: 10px;
      height: 100%;

    }
  }
  .form-selector{
    padding: 10px;
    background-color: $grey-light;
    border-radius: 10px;
    display: flex;
    button{
      border: 1px solid $grey-mid;
      border-left: none;
      border: right 1px solid $grey-mid;
      background-color: $grey-ultralight;
      color: $grey-dark;
      padding: 8px;
      &.active{
        color: $white;
        background-color: $secondary;
      }
      &:hover{
        cursor: pointer;
      }
      width: 100%;
      &:first-child{
        border-top-left-radius: 5px;
        border-bottom-left-radius: 5px;
      }
      &:last-child{
        border-top-right-radius: 5px;
        border-bottom-right-radius: 5px;
      }
    }
  }
  .mana-switch{
    display: grid;
    grid-row: 1fr 1fr 1fr;
    background-color: $grey-light;
    button{
      height: 35px;
      width: 35px;
      display: flex;
      border: none;
      align-items: center;
      justify-content: center;
      border: 1px solid $grey-mid;
      background-color: $grey-ultralight;
      &:hover{
        color: $white;
        background-color: $secondary;
      }
      &:first-child{
        border-top-left-radius: 5px;
        border-top-right-radius: 5px;
      }
      &:last-child{
        border-bottom-left-radius: 5px;
        border-bottom-right-radius: 5px;
      }
      &:hover{
        cursor: pointer;
      }
    }
    .icon-wrap{
      display: flex;
      justify-content: center;
      align-items: center;
      height: 35px;
      width: 35px;  
      border-left: 1px solid $grey-mid;
      border-right: 1px solid $grey-mid;
      background-color: $grey-ultralight;
    }
    .icon{
      color: $white;
      scale: 2;
    }
  }
  .label-input-grp{
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  form{
    min-width: 750px;
    border-radius: 10px;
    background-color: $grey-light;
    padding: 15px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    button{
      width: 100%;
    }
    .name-attack-defence{
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 10px;
      &.not-minion{
        grid-template-columns: 1fr;
      }
      .attack-defence{
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 10px;
      }
    }
    .manas-description{
      display: flex;
      flex-direction: row;
      gap: 10px;
      textarea{
        flex: 1;
        height: 100%;
      }
    }
    .manas{
      display: flex;
      justify-content: space-between;
    }
  }
  input, textarea, .file-upload{
    background-color: $grey-ultralight;
    border: 1px solid $grey-mid;
    padding: 10px;
    border-radius: 5px;
    &:focus{
      outline: 1px solid $secondary;
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

    min-height: 10rem;
    padding: 0px;

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
    height: 80%;
    width: 50%;
    object-fit: cover;
    border: none;
    border-radius: 3px;
    filter: drop-shadow(2px 2px 2px $grey-dark);
    
  }
</style>
