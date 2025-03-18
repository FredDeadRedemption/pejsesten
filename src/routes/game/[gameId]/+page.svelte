<script>
  import { page } from "$app/state";
  let gameId = $state(page.params.gameId);
</script>

<h1>Game ID: {gameId}</h1>