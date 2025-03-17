<script lang="ts">
  import { queueUp, socketLabel } from "$lib/socket/socket";

  let { profile } = $props();

  let message: string = $state("");
</script>

<main>
  <h1>Socket.IO Chat</h1>
  <p>Runnin on <strong>{socketLabel}</strong> change in <a href="/settings">settings</a></p>

  <button class="button primary" onclick={() => { queueUp("mista yehaw") }}>
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
