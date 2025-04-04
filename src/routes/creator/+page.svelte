<script lang="ts">
  import { enhance } from '$app/forms';
  import type { Database } from '$lib/database.types'; 

  let { data } = $props()
  let { land_enums, cards, profile} = $derived(data)

  type Card = Database['public']['Tables']['cards']['Row'];

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
</style>
