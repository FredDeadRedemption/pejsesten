<script lang="ts">
  import { enhance } from '$app/forms';
	import { getIcon } from '$lib/icons.js';
  import type { Database } from '$lib/database.types'; 
	import Card from '$lib/components/card.svelte';
	import CardSmall from '$lib/components/cardSmall.svelte';
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
    { color: "#bc874f", icon: "manaOrange", name: "orange" },
    { color: "#6e7f80", icon: "manaNeutral", name: "neutral" }
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
    neutral: 0,
  });

  const resetForm = () => {
    manaCounts.green = 0;
    manaCounts.orange = 0;
    manaCounts.red = 0;
    manaCounts.purple = 0;
    manaCounts.white = 0;
    manaCounts.black = 0;
    manaCounts.neutral = 0;
    cardImage = null;
    name = "";
    attack = null;
    defence = null;
    description = "";
  };

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
	  image_url: cardImage ?? "media/cards/missing-texture.jpg",
	  type: creating === "minion" ? 1 : creating === "mana" ? 2 : creating === "spell" ? 3 : 4,
	  neutral: manaCounts.neutral,
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

const increment = (name: string) => {
  if(name === "neutral"){
    if(manaCounts[name] < 15){
      manaCounts[name]++ 
    }
  } else if (manaCounts[name] < 3){
    manaCounts[name]++ 
  }
};

const chooseMana = (name: string) => {
  manaCounts.green = 0;
  manaCounts.orange = 0;
  manaCounts.red = 0;
  manaCounts.purple = 0;
  manaCounts.white = 0;
  manaCounts.black = 0;
  manaCounts[name] = 1;
};

const decrement = (name: string) => {
  if(manaCounts[name] > 0){
      manaCounts[name]-- 
    }
};


</script>

<main class="main">
  <!-- CREATE FORM -->
  <div class="create-wrap">
    <div class="form-selector">
      <button class="button" aria-label="minion" class:active={creating === "minion"} onclick={()=> { creating = "minion"; resetForm();}}>Create Minion</button>
      <button class="button" aria-label="mana" class:active={creating === "mana"} onclick={()=> { creating = "mana"; resetForm();}}>Create Mana</button>
      <button class="button" aria-label="spell" class:active={creating === "spell"} onclick={()=> { creating = "spell"; resetForm();}}>Create Spell</button>
      <button class="button" aria-label="incantation" class:active={creating === "incantation"} onclick={()=> { creating = "incantation"; resetForm();}}>Create Incantation</button>
    </div>
    {#if profile?.is_admin}
      <form id="create" method="POST" action="?/createCard" use:enhance enctype="multipart/form-data">
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
              {#if creating === "mana" && mana.name !== "neutral"}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div class="mana">
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <div class="mana-select  {mana.name}" class:active={manaCounts[mana.name] === 1} onclick={() => chooseMana(mana.name)}>
                    <div class="icon-wrap">
                      <span class="icon" style="color: {mana.color};">{@html getIcon(mana.icon)}</span>
                    </div>
                  </div>
                  <input bind:value={manaCounts[mana.name]} defaultValue="0" type="hidden" id={mana.name} name={mana.name} />
                </div>
              {:else if mana.name !== "neutral" || creating !== "mana"}
                <div class="mana">
                  <div class="mana-switch">
                    <button type="button" class="add" onclick={() => increment(mana.name)}>+</button>
                    <div class="icon-wrap">
                      <span class="icon" style="color: {mana.color};">{@html getIcon(mana.icon)}</span>
                    </div>
                    <button type="button" class="sub" onclick={() => decrement(mana.name)}>-</button>
                  </div>
                  <input bind:value={manaCounts[mana.name]} defaultValue="0" type="hidden" id={mana.name} name={mana.name} />
                </div>
              {/if}
            {/each} 
            <textarea bind:value={description} id="description" name="description" placeholder="Description"></textarea>
        </div>

        <div class="quad">
          <div class="img-model-animation">
            <div class="img">
              <div class="upload">
                <label for="file-upload" class="file-upload">
                  <span>{@html getIcon("photoPlus")}</span>
                </label>
                <input type="file" id="file-upload" name="image" accept="image/*" onchange={handleImgUpdate} />     
              </div>
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="unload" onclick={() => { cardImage = null; }}>
                <span>{@html getIcon("photoMinus")}</span>
              </div>
            </div>
            <div class="inputs">
              <input type="text" id="model" name="model" placeholder="Model" />
              <select id="animation" name="animation" placeholder="Animation">
                <option value="null">No animation</option>
                <option value="melee">Melee</option>
                <option value="range">Ranged</option>
            </div>
          </div>
          <div class="tips"></div>
          <div class="buttons-debug">
            <div class="buttons">
              <button type="submit" class="button primary" onclick={() => setTimeout(()=>{
                resetForm();
              }, 350)}>Create Card</button>
              <button type="button" class="button" onclick={resetForm}>Reset</button>
            </div>
            <div class="debug">
              debug console
            </div>
          </div>
        </div>
      </form>
    {:else}
      <h1>Only admin users can use this page :(</h1>
    {/if}
  </div>
  <div class="showcase">
    <Card card={testCard} />
    {#if creating ==="minion" || creating === "mana"}
      <CardSmall card={testCard} />
    {/if}
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
      display: flex;
      flex-direction: column;
      justify-content: space-evenly;
      align-items: center;
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
  .mana-select{
    background-color: $grey-ultralight;
    border-radius: 5px;
    border: 1px solid $grey-mid;
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 105px;
    width: 35px;
    &:hover{
      cursor: pointer;
    }
    .icon-wrap{
      display: flex;
      justify-content: center;
      align-items: center;
      height: 35px;
      width: 35px;  
      background-color: $grey-ultralight;
    }
    .icon{
      color: $white;
      scale: 2;
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
        resize: none;
        flex: 1;
        height: 100%;
      }
    }
    .manas{
      display: flex;
      justify-content: space-between;
    }
    .quad{
      display: grid;
      grid-template-columns: 1fr 2fr 1fr;
      gap: 10px;
    }
  }
  input, textarea, select {
    background-color: $grey-ultralight;
    border: 1px solid $grey-mid;
    padding: 10px;
    border-radius: 5px;
    &:focus{
      outline: 1px solid $secondary;
    }
  }

  input[type="number"]::-webkit-inner-spin-button,
  input[type="number"]::-webkit-outer-spin-button {
    -webkit-appearance: none; /* WebKit browsers */
    margin: 0; /* Optional: Remove margin */
  }

  input[type="file"] {
    display: none;
  }

  .img-model-animation{
    display: grid;
    gap: 10px;
    grid-template-rows: 1fr 1fr;
    aspect-ratio: 1;
  }
  .img{
    gap: 10px;
    display: grid;
    grid-template-columns: 1fr 1fr;
  }
  .inputs{
    display: grid;
    gap: 10px;
    grid-template-rows: 1fr 1fr;
  }
  .file-upload, .upload, .unload {
    background-color: $grey-ultralight;
    border: 1px solid $grey-mid;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: $grey-mid;
    &:hover{
      color: $secondary;
      border-color: $secondary;
      .file-upload{
        color: $secondary;
      }
    }
  }
  .file-upload{
    border: none;
  }
  .tips{
    padding: 0px;
    background-color: $grey-light;
    border: 1px solid $grey-mid;
    border-radius: 5px;
  }
  .buttons-debug{
    display: grid;
    grid-template-rows: 1fr 1fr;
    gap: 10px;
  }
  .buttons{
    display: grid;
    grid-template-rows: 1fr 1fr;
    gap: 10px;
    button{
      border-radius: 5px;
      &:last-child{
        transition: ease all 250ms;
        background-color: $grey-dark;
        color: $white;
        border: none;
        &:hover{
          cursor: pointer;
          background-color: $grey-mid;
        }
      }
    }
  }
  .debug{
    padding: 10px;
    background-color: $grey-mid;
    color: $info;
    border-radius: 5px;
  }
    .green {
      &.active {
        background-color: $mana-green;
        .icon-wrap { 
          background-color: $mana-green;
          .icon{
            color: $white !important;
          }
        }
      }
    }
    .orange {
      &.active {
        background-color: $mana-orange;
        .icon-wrap { 
          background-color: $mana-orange;
          .icon{
            color: $white !important;
          }
        }
      }
    }
    .red {
      &.active {
        background-color: $mana-red;
        .icon-wrap { 
          background-color: $mana-red;
          .icon{
            color: $white !important;
          }
        }
      }
    }
    .purple {
      &.active {
        background-color: $mana-purple;
        .icon-wrap { 
          background-color: $mana-purple;
          .icon{
            color: $white !important;
          }
        }
      }
    }
    .white {
      &.active {
        background-color: $white;
        .icon-wrap { 
          background-color: $white;
          .icon{
            color: $mana-black !important;
          }
        }
      }
    }
    .black {
      &.active {
        background-color: $mana-black;
        .icon-wrap { 
          background-color: $mana-black;
          .icon{
            color: $white !important;
          }
        }
      }
    }
</style>
