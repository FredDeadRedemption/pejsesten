<script lang="ts">
	import { gameState, tradeCard } from '$lib/socket/socket.svelte';
	import { drag } from '$lib/drag.svelte';
	import { targeting } from '$lib/targeting.svelte';
	import { isTradeable } from '$lib/shared/lib';

	const onDrop = () => {
		if (targeting.active) return
		if (!drag.card || drag.index === null) return;
		if (!isTradeable(drag.card)) return;
		drag.consumed = true;
		tradeCard({ index: drag.index });
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div id="wrapper">
	<div
		id="deck"
		class:glow-white={drag.card && isTradeable(drag.card) && !targeting.active && gameState.self_board.mana !== 0}
		onmouseup={onDrop}
	>
		DECK | {gameState.self_board.deck.length}
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