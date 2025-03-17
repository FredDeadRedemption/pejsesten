<script lang="ts">
  import { socket } from "$lib/socket/socketStore";
  import { toggleLabel } from "$lib/socket/toggleStore";

  let { profile } = $props();

  let message: string = $state("");

  let messages = $derived($socket.messages)
</script>

<main>
  <h1>Socket.IO Chat</h1>
  <p>Runnin on <strong>{$toggleLabel}</strong> change in <a href="/settings">settings</a></p>
  <div>
    <input
    type="text"
    bind:value={message}
    placeholder="Type a message"
    onkeydown={(e) => e.key === 'Enter' && $socket.sendMessage(message)}
  />
  <button onclick={() => { $socket.sendMessage(message); message = ""; }}>
    Send
  </button>
  </div>

  <button onclick={() => { $socket.queueUp("mista yehaw") }}>
    QueueUp
  </button>
  <ul>
    {#each $messages as msg}
      <li class="msg">{msg}</li>
    {/each}
  </ul>
</main>

<style lang="scss">
  a{
    color: $primary;
  }
  .msg{
    width: 500px;
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
