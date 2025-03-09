<script lang="ts">
  import "../style.scss"
  import { invalidate } from '$app/navigation'
  import { onMount } from 'svelte'
	import Nav from "$lib/nav.svelte";
	import { page } from '$app/state'
	import Dock from "$lib/dock.svelte";

  let { data, children } = $props()
  let { session, supabase } = $derived(data)

  onMount(() => {
    const { data } = supabase.auth.onAuthStateChange((_, newSession) => {
      if (newSession?.expires_at !== session?.expires_at) {
        invalidate('supabase:auth')
      }
    })

    return () => data.subscription.unsubscribe()
  })
</script>

{#if page.url.pathname !== "/"}
  <Nav session={session} supabase={supabase}> </Nav>
  <Dock></Dock>
  <main>
    {@render children()}
  </main>
{:else}
  {@render children()}
{/if}

<style lang="scss">
  main{
    margin-top: 50px;
    margin-left: 50px;
    box-shadow: inset 0 10px 10px -10px rgba(0, 0, 0, 0.25);
    background-color: $grey-light;
    min-width: calc(100vw - 50px);
    min-height: calc(100vh - 50px);
    padding: 25px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
</style>