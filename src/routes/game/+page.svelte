<script lang="ts">
  import { queueUp, invalidateSocket, connectSocket } from "$lib/socket/socket";

  let { data } = $props()
  let { profile } = $derived(data)
    
  let dev = $state(false)

  let url = $derived(dev ? "http://localhost:3000/" : "https://matrixz-gs.up.railway.app/");

  $effect(()=>{
    console.log("url changed invalidating socket " + url) 
    invalidateSocket();
  })
</script>

<main>
  <h1>Socket.IO Chat</h1>
  <p>Runnin on <strong>{dev ? "Development" : "Production"}</strong> change in <a href="/settings">settings</a></p>
  <p>{profile?.username}</p>
  <input type="checkbox" name="url" id="" bind:checked={dev}>

  <button class="button primary" onclick={() => { 
    connectSocket(url);
    queueUp(profile?.username || "Out-of-Towner") 
    }}>
    QueueUp
  </button>
</main>

<style lang="scss">
  a{
    color: $primary;
  }
  main {
    display: flex;
    flex-direction: column;
    width: 100%;
    align-items: center;
    text-align: center;
    margin-top: 2rem;
    gap: 10px;
  }
</style>
