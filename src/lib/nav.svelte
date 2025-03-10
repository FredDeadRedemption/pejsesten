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
  <a class="home button ghost" href="/dashboard">HOME</a>
  <button class="signin-out button ghost" onclick={logout}>LOGOUT</button>
  <!-- svelte-ignore a11y_consider_explicit_label -->
  <a href="profile"><div class="avatar"></div></a>
</nav>

<style lang="scss">
  #nav{
    position: fixed;
    box-shadow: 0 10px 10px -10px rgba(0, 0, 0, 0.25);
    border-bottom: 1px solid $grey-mid;
    z-index: 2;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-end;
    gap: 25px;
    padding: 0px 30px 0px 30px;
    background-color: $white;
    width: 100%;
    height: 50px;
    z-index: 99;
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
      background-color: $grey-black;
      height: 40px;
      width: 40px;
      border-radius: 100px;
    }
  }
</style>
