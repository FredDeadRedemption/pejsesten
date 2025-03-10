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
  input:-webkit-autofill,
input:-webkit-autofill:hover, 
input:-webkit-autofill:focus,
textarea:-webkit-autofill,
textarea:-webkit-autofill:hover,
textarea:-webkit-autofill:focus,
select:-webkit-autofill,
select:-webkit-autofill:hover,
select:-webkit-autofill:focus {
  -webkit-text-fill-color: $primary;
  -webkit-box-shadow: 0 0 0px 1000px transparent inset;
  transition: background-color 5000s ease-in-out 0s;
}
  form{
    display: flex;
    flex-direction: column;
    padding: 25px;
    gap: 25px;
    background-color: $white;
    backdrop-filter: blur(5px);
    border: 1px solid $grey-mid;
    input, button{
      width: 250px;
      padding: 10px;
    }
    input{  
      border: none;
      border: 1px solid $grey-mid;
      background-color: $grey-ultralight;  
      outline: none;
      color: $grey-mid;
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
    transition: 250ms ease all;
    background-color: $primary;
    color: $white;
    &:hover{
      background-color: $black;
    }
  }
  .create{
    transition: 250ms ease all;
    border: 1px solid $grey-mid;
    background-color: $white;
    &:hover{
      background-color: $grey-mid;
      color: $white;
    }
  }
</style>