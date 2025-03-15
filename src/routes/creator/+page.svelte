<script lang="ts">
  import { enhance } from '$app/forms';
</script>

<form method="POST" action="?/createCard" use:enhance enctype="multipart/form-data">
  <label for="name">Name:</label>
  <input type="text" id="name" name="name" required />

  <label for="attack">Attack:</label>
  <input type="number" id="attack" name="attack" required />

  <label for="defence">Defence:</label>
  <input type="number" id="defence" name="defence" required />

  <label for="description">Description:</label>
  <textarea id="description" name="description"></textarea>

  <label for="image">Image:</label>
  <input type="file" id="image" name="image" accept="image/*" />

  <button type="submit" class="button primary">Create Card</button>
</form>

<form method="POST" action="?/deleteCard" use:enhance>
  <label for="name">Name:</label>
  <input type="text" id="name" name="name" required />

  <button type="submit" class="button primary">Delete Card</button>
</form>

<style lang="scss">

</style>
