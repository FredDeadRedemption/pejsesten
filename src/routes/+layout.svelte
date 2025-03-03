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
  <button class="logout" onclick={logout}>Logout</button>
</nav>

{@render children()}

<style lang="scss">
  #nav{
    display: flex;
    flex-direction: row;  
    align-items: center;
    justify-content: space-between;
    height: 80px;
    width: 100vw;
    padding: 25px;
    gap: 10px;
    background-color: $grey-mid;
    box-shadow: $box-shadow-primary;
    .logout{
      border: 1px solid crimson;
      background-color: rgb(224, 64, 96);
      color: $white;
      padding: 15px;
      &:hover{
        cursor: pointer;
        background-color: crimson;
      }
    }
  }
</style>