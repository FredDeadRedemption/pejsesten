<script lang="ts">
  import "../style.scss"
  import { invalidate } from '$app/navigation'
  import { onMount } from 'svelte'
	import Nav from "$lib/components/nav.svelte";
	import { page } from '$app/state'
	import Dock from "$lib/components/dock.svelte";
	import ProgressBar from "$lib/components/progressBar.svelte";

  let { data, children } = $props()
 
</script>

{#if page.url.pathname !== "/"}
  <ProgressBar></ProgressBar>
  <Nav> </Nav>
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
    background-color: rgb(91, 110, 140);
    min-width: calc(100vw - 50px);
    min-height: calc(100vh - 50px);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
</style>