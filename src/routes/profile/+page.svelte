<!-- src/routes/account/+page.svelte -->
<script lang="ts">
   let { data } = $props()
   let { profile } = $derived(data)
</script>

<div>{profile?.first_name}</div>
