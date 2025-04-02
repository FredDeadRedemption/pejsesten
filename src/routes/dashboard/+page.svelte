<script lang="ts">
  let { data } = $props()
  let { profile } = $derived(data)

  // TODO FIX DET HER LORT
  $effect(() => {
    if (profile == null) {
      reload();
    }
  });

  const reload = () => {
    if (!browser) return;
    window.location.reload();
  }

  import { invalidate } from '$app/navigation';
  import { onMount } from 'svelte';
  import { page } from '$app/state';
	import { browser } from '$app/environment';

  let refresh = page.url.searchParams.get('refresh') === 'true';

  // Invalidate data if the query parameter is present
  onMount(async () => {
    if (refresh) {
      await invalidate('app:data'); // Re-run load functions
    }
  });
</script>

<main class="main">
  <h1>dashboard</h1>
  <p>Greetings, <strong>{profile?.username}!</strong> 
    Take a look at the <a href="/catalog">Catalog</a> 
  or browse the <a href="/deck-builder">Deck Builder</a>,
  you have <span class="wins">0</span> wins!&nbsp;(noob)</p>
</main>

<style lang="scss">
  .main{
    padding: 30px;
  }
  p{ 
    color: $black;
  }
  .wins{
    color: $primary;
  }
  a{
    color: $primary;
  }
</style>