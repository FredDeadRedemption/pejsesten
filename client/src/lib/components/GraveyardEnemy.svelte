<script lang="ts">
	import { gameState } from '$lib/socket.svelte';
</script>

<div id="wrapper">
	<div id="graveyard">
		{gameState.enemy_board.graveyard.length}
	</div>
</div>

<style lang="scss">
	@use '../../vars' as *;
	#wrapper {
		height: 100%;
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		#graveyard {
			height: 147px;
			border: 3px solid grey;
			width: 100px;
			&:hover {
				cursor: pointer;
			}
		}
	}
</style>
