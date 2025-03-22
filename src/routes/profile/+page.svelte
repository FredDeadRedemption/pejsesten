<script lang="ts">
	import { enhance } from '$app/forms';
	import { onMount } from 'svelte';

   let { data } = $props()
   let { profile } = $derived(data)
   let username: any = $state("");
   let originalUsername: any = $state("");

   onMount(()=>{
      username = profile?.username
      originalUsername = profile?.username
   })
 </script>
 
 <main class="main">
   <form use:enhance={() => {
      return ({ result }) => {
         if(!result) return;
         if (result.type === 'success') {  
            username = result?.data?.username;
            originalUsername = result?.data?.username;
         }
      };
    }} action="?/update" method="post">
     <label for="username">Hello there, <b>{originalUsername}</b>, brave adventurer! Forge your destiny and customize your profile here.</label>
     <input type="text" id="username" name="username" bind:value={username} required />
     <button class="button primary" type="submit">Update</button>
   </form>
 </main>
 
 <style lang="scss">
   .main {
      margin: 30px auto;
      max-width: 400px;
      text-align: center;
   }
   input {
      text-align: center;
      font-size: 2rem;
      background-color: transparent;
      width: 100%;
      padding: 8px;
      margin: 10px 0;
      border: none;
      border-bottom: 1px solid $grey-mid;
   }
   button{
      width: 100% !important;
   }
 </style>
 