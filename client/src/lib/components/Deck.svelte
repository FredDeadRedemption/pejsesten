<script lang="ts">
	import { gameState, tradeCard } from '$lib/socket.svelte';
	import { drag } from '$lib/drag.svelte';
	import { targeting } from '$lib/targeting.svelte';
	import { isTradeable } from '$lib/lib';
	import { getEntity } from '$lib/lib';

	const getCardFromBoard = (i: number) => {
		const raw = gameState.self_board.hand[i];
		if (!raw) return;
		return getEntity(raw);
	};

	const onDrop = () => {
		if (targeting.active) return;
		if (drag.index === null) return;
		const card = getCardFromBoard(drag.index);
		if (!isTradeable(card!)) return;
		drag.consumed = true;
		tradeCard({ index: drag.index });
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div id="wrapper">
	<div
		id="deck"
		class:glow-white={drag.index &&
			isTradeable(getCardFromBoard(drag.index)!) &&
			!targeting.active &&
			gameState.self_board.mana !== 0}
		onmouseup={onDrop}
	>
		DECK | {gameState.self_board.deck.length}
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
