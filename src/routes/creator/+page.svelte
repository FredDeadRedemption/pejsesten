<script lang="ts">
  import { enhance } from '$app/forms';
  
  import type { Database } from '$lib/database.types'; 


  let { data } = $props()
  let { land_enums, cards, profile} = $derived(data)


  type Card = Database['public']['Tables']['cards']['Row'];

  let cardToUpdate : Card | null = $state(null); 



  const handleImgUpdate = (event : Event) => {
    const target = event.target as HTMLInputElement;

    const img = target.files?.[0];

    if (img) {
      const reader = new FileReader();

      reader.onload = (e) => {
        if(cardToUpdate) {
          cardToUpdate.image_url = e.target?.result as string;
          console.log(cardToUpdate.image_url)
        }
      }
      reader.readAsDataURL(img);

      // console.log(img);
    }
  } 

</script>

<main class="main">
  <!-- CREATE FORM -->
   {#if profile?.is_admin}
  <form id="create" method="POST" action="?/createCard" use:enhance enctype="multipart/form-data">
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
        <label for="holyCost">Holy</label>
        <input type="number" id="holyCost" name="holyCost" />
      </div>

      <div class="grp">
        <label for="deathCost">Death</label>
        <input type="number" id="deathCost" name="deathCost"/>
      </div>

      <div class="grp">
        <label for="dreamCost">Dream</label>
        <input type="number" id="dreamCost" name="dreamCost" />
      </div>

      <div class="grp">
        <label for="earthCost">Earth</label>
        <input type="number" id="earthCost" name="earthCost" />
      </div>
    </div>

    <div class="grp">
      <label for="description">Description</label>
      <textarea id="description" name="description"></textarea>
    </div>

    <div class="grp">
      <label for="image">Image</label>
      <input type="file" id="image" name="image" accept="image/*" />
    </div>

    <button type="submit" class="button primary">Create Card</button>
  </form>


  <!-- DELETE FORM -->
  <form id="delete" method="POST" action="?/deleteCard" use:enhance>
    <div class="grp">
      <label for="name">Name</label>
      <select name="name">
        <option value="" disabled selected>Select an option</option>
        <!-- Land -->
        <!-- <optgroup label="Land"> TODO: Should be land
          {#each cards as card}
            <option value={card.name}>{card.name}</option>
          {/each}
        </optgroup> -->

        <!-- Creatures -->
        <optgroup label="Creatures">
          {#each cards as card}
            <option value={card.name}>{card.name}</option>
          {/each}
        </optgroup>
        
        <!-- Incantations -->
        <!-- <optgroup label="Land"> TODO: Should be incantations(spells)
          {#each cards as card}
            <option value={card.name}>{card.name}</option>
          {/each}
        </optgroup> -->
      </select>
    </div>

    <button type="submit" class="button primary">Delete Card</button>
  </form>


  <!-- UPDATE FORM -->
  <div id="update">
  <!-- <form action=}> -->
    <select name="name" bind:value={cardToUpdate}>
      <option value="" disabled selected>Select an option</option>
      <!-- Land -->
      <!-- <optgroup label="Land"> TODO: Should be land
        {#each cards as card}
          <option value={card.name}>{card.name}</option>
        {/each}
      </optgroup> -->

      <!-- Creatures -->
      <optgroup label="Creatures">
        {#each cards as card}
          <option value={card}>{card.name}</option>
        {/each}
      </optgroup>
      
      <!-- Incantations -->
      <!-- <optgroup label="Land"> TODO: Should be incantations(spells)
        {#each cards as card}
          <option value={card.name}>{card.name}</option>
        {/each}
      </optgroup> -->
    </select>
    <!-- <button onclick={()=>console.log(cardToUpdate)}>yo</button> -->
  <!-- </form> -->

  {#if cardToUpdate}
  <form  method="POST" action="?/updateCard" use:enhance enctype="multipart/form-data">
    <div class="grp">
      <label for="name">Name</label>
      <input type="text" id="name" name="name" required bind:value={cardToUpdate.name}/>
    </div>

    <div class="grp-side-by-side-four">
      <div class="grp">
        <label for="holyCost">Holy</label>
        <input type="number" id="holyCost" name="holyCost" bind:value={cardToUpdate.holy_cost}/>
      </div>

      <div class="grp">
        <label for="deathCost">Death</label>
        <input type="number" id="deathCost" name="deathCost" bind:value={cardToUpdate.death_cost}/>
      </div>

      <div class="grp">
        <label for="dreamCost">Dream</label>
        <input type="number" id="dreamCost" name="dreamCost" bind:value={cardToUpdate.dream_cost}/>
      </div>

      <div class="grp">
        <label for="earthCost">Earth</label>
        <input type="number" id="earthCost" name="earthCost" bind:value={cardToUpdate.earth_cost}/>
      </div>
    </div>

    <div class="grp">
      <label for="description">Description</label>
      <textarea id="description" name="description" bind:value={cardToUpdate.description}></textarea>
    </div>

    <div class="grp">
      <label for="image">Image</label>
      <input type="file" id="image" name="image" accept="image/*" onchange={handleImgUpdate} />
      {console.log(cardToUpdate.image_url)}
      <img src={cardToUpdate.image_url} alt="" draggable="false">
    </div>
    <button type="submit" class="button primary">Update Card</button>
  </form>
  {/if}
  </div>
  {#if cardToUpdate}

  <img src={cardToUpdate.image_url} alt="" draggable="false">
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
  #create, #delete, #update{
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
  input, textarea{
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
  img {
    z-index: 1;
    height: 100%;
    width: 100%;
    object-fit: cover;
  }
</style>
