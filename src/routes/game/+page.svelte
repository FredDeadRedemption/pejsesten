<script lang="ts">
  import { sendMessage, queueUp, messages, socketLabel } from "$lib/socket/socket";

  let { profile } = $props();

  let message: string = $state("");
</script>

<main>
  <h1>Socket.IO Chat</h1>
  <p>Runnin on <strong>{socketLabel}</strong> change in <a href="/settings">settings</a></p>
  <div>
    <input
    type="text"
    bind:value={$messages}
    placeholder="Type a message"
    onkeydown={(e) => e.key === 'Enter' && sendMessage(message)}
  />
  <button onclick={() => { sendMessage(message); message = ""; }}>
    Send
  </button>
  </div>

  <button onclick={() => { queueUp("mista yehaw") }}>
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
