<script lang="ts">
  import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { slide } from 'svelte/transition';

  let errmsg = $state();
</script>

<div class="content">
  <form method="POST" action="?/login" use:enhance={() => {
		return async ({ result }: any) => {
      console.log(result);
      if(result.type = "redirect"){
        goto(result.location);
      }
      if(!result?.data?.success){
        errmsg = result?.data?.message;
      } 
		};
	}}>
    <input name="email" type="email" placeholder="EMAIL" />
    <input name="password" type="password" placeholder="PASSWORD" />
    <button class="login">LOGIN</button>
    <button class="create" formaction="?/signup">SIGN UP</button>
  </form>
  
  {#if errmsg}
    <div transition:slide={{ axis: "y", duration: 500}} class="status">
      <div>{errmsg}</div>  
    </div>
  {/if}
</div>

<style lang="scss">
  .status{
    margin-top: 25px;
    color: $warning;
    border-radius: 4px;
    padding: 25px;
    background-color: rgba($color: $warning, $alpha: 0.2);
    backdrop-filter: blur(5px);
    border: 1px solid $warning;
  }
  .content{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 300px;
  }
  form{
    display: flex;
    flex-direction: column;
    border-radius: 4px;
    padding: 25px;
    gap: 25px;
    background-color: rgba($color: $white, $alpha: 0.2);
    backdrop-filter: blur(5px);
    border: 1px solid $white;
    input, button{
      width: 250px;
      padding: 10px;
      border-radius: 4px;
    }
    input{  
      border: 1px solid $white;
      background-color: transparent;  
      outline: none;
      color: $white;
    }
    button{
      border: none;
      color: $black;
      &:hover{
        cursor: pointer;
      }
    }
  }
  .login{
    background-color: $ok;
    color: $white;
  }
  .create{
    background-color: $white;
  }
</style>