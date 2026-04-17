<script lang="ts">
	import { gameState } from '$lib/socket/socket.svelte';
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div id="wrapper">
	<div id="deck">
		DECK | {gameState.enemy_board.deck.length}
	</div>
</div>

<style lang="scss">
	#wrapper {
		height: 100%;
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		#deck {
			height: 147px;
			border: 3px solid grey;
			width: 100px;
			&:hover {
				cursor: pointer;
			}
		}
	}
</style>
