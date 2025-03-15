<script lang="ts">
  import { messages, sendMessage } from "$lib/socket";
  import { onMount } from "svelte";

  let message = $state("");
</script>

<main>
  <h1>Socket.IO Chat</h1>

  <input
    type="text"
    bind:value={message}
    placeholder="Type a message"
    onkeydown={(e) => e.key === 'Enter' && sendMessage(message)}
  />
  <button onclick={() => { sendMessage(message); message = ""; }}>
    Send
  </button>

  <ul>
    {#each $messages as msg}
      <li>{msg}</li>
    {/each}
  </ul>
</main>

<style lang="scss">
  main {
    text-align: center;
    margin-top: 2rem;
  }
  input {
    padding: 0.5rem;
    margin-right: 0.5rem;
  }
  ul {
    list-style: none;
    padding: 0;
  }
  li {
    background: #f0f0f0;
    padding: 0.5rem;
    margin: 0.25rem 0;
    border-radius: 5px;
  }
</style>
