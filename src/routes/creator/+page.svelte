<script lang="ts">
  import { enhance } from '$app/forms';
</script>

<main>
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

    <div class="grp-side-by-side">
      <div class="grp">
        <label for="holy_cost">Holy cost</label>
        <input type="number" id="holy_cost" name="holy_cost" required />
      </div>

      <div class="grp">
        <label for="death_cost">Death cost</label>
        <input type="number" id="death_cost" name="death_cost" required />
      </div>
    </div>

    <div class="grp-side-by-side">
      <div class="grp">
        <label for="dream_cost">Dream Cost</label>
        <input type="number" id="dream_cost" name="dream_cost" required />
      </div>

      <div class="grp">
        <label for="earth_cost">Earth Cost</label>
        <input type="number" id="earth_cost" name="earth_cost" required />
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

  <form id="delete" method="POST" action="?/deleteCard" use:enhance>
    <div class="grp">
      <label for="name">Name</label>
      <input type="text" id="name" name="name" required />
    </div>

    <button type="submit" class="button primary">Delete Card</button>
  </form>

  <form id="update" method="POST" action="?/updateCard" use:enhance enctype="multipart/form-data">
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

    <div class="grp-side-by-side">
      <div class="grp">
        <label for="holy_cost">Holy cost</label>
        <input type="number" id="holy_cost" name="holy_cost" required />
      </div>

      <div class="grp">
        <label for="death_cost">Death cost</label>
        <input type="number" id="death_cost" name="death_cost" required />
      </div>
    </div>

    <div class="grp-side-by-side">
      <div class="grp">
        <label for="dream_cost">Dream Cost</label>
        <input type="number" id="dream_cost" name="dream_cost" required />
      </div>

      <div class="grp">
        <label for="earth_cost">Earth Cost</label>
        <input type="number" id="earth_cost" name="earth_cost" required />
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

    <button class="button secondary">Load Card</button>
    <button type="submit" class="button primary">Update Card</button>
  </form>
</main>


<style lang="scss">
  main{
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    justify-content: center;
    align-items: center;
    gap: 25px;
  }
  #create, #delete, #update{
    border-radius: 6px;
    background-color: $grey-light;
    padding: 15px;
    display: flex;
    flex-direction: column;
    gap: 15px;
    input, textarea{
      background-color: $grey-mid;
      border: none;
      padding: 10px;
      border-radius: 4px;
      &:focus{
        outline: none;
      }
    }
    button{
      width: 100%;
    }
    textarea{
      height: 70px;
    }
  }
  .basics{
    display: grid;
    grid-template-rows: 1fr 1fr 1r
  }
  .grp{
    display: flex;
    flex-direction: column;
  }
  .grp-side-by-side{
    display: flex;
    flex-direction: row;
    gap: 15px;
  }
  label{
    align-self: center;
  }
</style>
