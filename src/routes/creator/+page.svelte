<script lang="ts">
  import { enhance } from '$app/forms';
	import { getIcon } from '$lib/icons.js';

  let { data } = $props()
  let { profile} = $derived(data)

  type creatingT = "minion" | "mana" | "spell";

  let creating: creatingT = $state("minion");
  let cardImage : string | null = $state(null);

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
   <div class="buttons">
    <button class="button primary" aria-label="minion" onclick={()=> { creating = "minion"; cardImage = null;}}>Create Minion</button>
    <button class="button primary" aria-label="mana" onclick={()=> { creating = "mana"; cardImage = null;}}>Create Mana</button>
    <button class="button primary" aria-label="spell" onclick={()=> { creating = "spell"; cardImage = null;}}>Create Spell</button>
   </div>
  {#if profile?.is_admin}
  {#if creating === "minion"}
    <form id="create" method="POST" action="?/createCard" use:enhance enctype="multipart/form-data" onsubmit={()=>(cardImage=null)}>
      <h1>CREATE MINION</h1>
      <input type="hidden" id="type" name="type" value="1" required>

      <div class="grp">
        <label for="name">Name</label>
        <input type="text" id="name" name="name" required />
      </div>

      <div class="grp-side-by-side">
        <div class="grp">
          <label for="attack">Attack</label>
          <input type="number" id="attack" name="attack" required />
        </div>
    
        <div class="grp">
          <label for="defence">Defence</label>
          <input type="number" id="defence" name="defence" required />
        </div>
      </div>

      <div class="grp-side-by-side-four">
        <div class="grp">
          <label for="white">White</label>
          <input defaultValue="0" type="number" id="white" name="white" />
        </div>

        <div class="grp">
          <label for="black">Black</label>
          <input defaultValue="0" type="number" id="black" name="black"/>
        </div>

        <div class="grp">
          <label for="purple">Purple</label>
          <input defaultValue="0" type="number" id="purple" name="purple" />
        </div>

        <div class="grp">
          <label for="green">Green</label>
          <input defaultValue="0" type="number" id="green" name="green" />
        </div>

        <div class="grp">
          <label for="red">Red</label>
          <input defaultValue="0" type="number" id="red" name="red" />
        </div>

        <div class="grp">
          <label for="orange">Orange</label>
          <input defaultValue="0" type="number" id="orange" name="orange" />
        </div>
      </div>

      <div class="grp">
        <label for="description">Description</label>
        <textarea id="description" name="description"></textarea>
      </div>

      <div class="grp">
        <label for="image">Image</label>
        <label for="file-upload" class="file-upload">
          
          {#if cardImage}
            <img class="image" src={cardImage} alt="" draggable="false">
          {/if}
          <span>{@html getIcon("new")}</span>
        
        </label>
        <input type="file" id="file-upload" name="image" accept="image/*" onchange={handleImgUpdate} />
      </div>

      <button type="submit" class="button primary">Create Card</button>
    </form>
  {/if}
  {#if creating === "mana"}
    <form id="create" method="POST" action="?/createCard" use:enhance enctype="multipart/form-data" onsubmit={()=>(cardImage=null)}>
      <h1>CREATE MANA</h1>
      <input type="hidden" id="type" name="type" value="2" required>

      <div class="grp">
        <label for="name">Name</label>
        <input type="text" id="name" name="name" required />
      </div>

      <div class="grp-side-by-side-four">
        <div class="grp">
          <label for="white">White</label>
          <input defaultValue="0" type="number" id="white" name="white" />
        </div>

        <div class="grp">
          <label for="black">Black</label>
          <input defaultValue="0" type="number" id="black" name="black"/>
        </div>

        <div class="grp">
          <label for="purple">Purple</label>
          <input defaultValue="0" type="number" id="purple" name="purple" />
        </div>

        <div class="grp">
          <label for="green">Green</label>
          <input defaultValue="0" type="number" id="green" name="green" />
        </div>

        <div class="grp">
          <label for="red">Red</label>
          <input defaultValue="0" type="number" id="red" name="red" />
        </div>

        <div class="grp">
          <label for="orange">Orange</label>
          <input defaultValue="0" type="number" id="orange" name="orange" />
        </div>
      </div>

      <div class="grp">
        <label for="description">Description</label>
        <textarea id="description" name="description"></textarea>
      </div>

      <div class="grp">
        <label for="image">Image</label>
        <label for="file-upload" class="file-upload">
          
          {#if cardImage}
            <img class="image" src={cardImage} alt="" draggable="false">
          {/if}
          <span>{@html getIcon("new")}</span>
        
        </label>
        <input type="file" id="file-upload" name="image" accept="image/*" onchange={handleImgUpdate} />
      </div>

      <button type="submit" class="button primary">Create Card</button>
    </form>
    {/if}
  {:else}
    <h1>Only admin users can use this page :(</h1>
  {/if}
</main>


<style lang="scss">
  .main{
    margin: 30px;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    justify-content: center;
    gap: 25px;
  }
  .buttons{
    display: flex;
    gap: 10px;
    flex-direction: column;
  }
  #create{
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
  .grp-side-by-side{
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 15px;
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
