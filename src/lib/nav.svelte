<script>
	import { goto } from "$app/navigation";

  let { supabase, session } = $props();
  
  const logout = async () => {
    const { error } = await supabase.auth.signOut()
    if (error) {
      console.error(error)
    } else{
      goto("/")
    }
  }
</script>

<nav id="nav">
  <a class="home button ghost" href="/">HOME</a>
  {#if session}
    <button class="signin-out button ghost" onclick={logout}>LOGOUT</button>
  {:else}
    <a class="signin-out button ghost" href="/auth">lOGIN</a>
  {/if}
  <div class="avatar"></div>
</nav>

<style lang="scss">
  #nav{
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-end;
    gap: 25px;
    padding: 0px 30px 0px 30px;
    background-color: $white;
    width: 100%;
    height: 50px;
    .home{
      position: absolute;
      left: 25px;
      color: $black;
      background-color: inherit;
    }
    .signin-out{
      color: $black;
    }
    .avatar{
      height: 40px;
      width: 40px;
      border-radius: 100px;
    }
  }
</style>
