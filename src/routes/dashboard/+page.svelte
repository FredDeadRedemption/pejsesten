<script lang="ts">
  let { data } = $props()
  let { land_enums } = $derived(data)
</script>

<h1>Welcome to Supabase!</h1>
<ul>
  {#each land_enums as le}
    <li>{le.name}</li>
  {/each}
</ul>