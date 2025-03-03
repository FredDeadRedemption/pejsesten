<script>
  import "../style.scss"
  import { invalidate } from '$app/navigation'
  import { onMount } from 'svelte'

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

  const logout = async () => {
    const { error } = await supabase.auth.signOut()
    if (error) {
      console.error(error)
    }
  }
</script>

<nav id="nav">
  <a href="/">Home</a>
  {#if session}
    <button class="auth" onclick={logout}>Logout</button>
  {:else}
    <a class="auth" href="/auth">login</a>
  {/if}
</nav>

{@render children()}

<style lang="scss">
  #nav{
    position: absolute;
    display: flex;
    flex-direction: row;  
    align-items: center;
    justify-content: space-between;
    height: 80px;
    margin: 10px;
    width: calc(100% - 20px);
    left: 50%;
    transform: translateX(calc(-50% - 10px));
    padding: 25px;
    gap: 10px;
    background-color: $grey-mid;
    box-shadow: $box-shadow-primary;
    .auth{
      border: 1px solid crimson;
      background-color: rgb(224, 64, 96);
      color: $white;
      padding: 15px;
      text-decoration: none;
      &:hover{
        cursor: pointer;
        background-color: crimson;
      }
    }
  }
</style>