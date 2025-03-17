<script>
  import { page } from "$app/stores";
  $: gameId = $page.params.gameId;
</script>

<h1>Game ID: {gameId}</h1>