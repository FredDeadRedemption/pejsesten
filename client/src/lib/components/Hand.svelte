<script lang="ts">
	import { tick } from 'svelte';
	import Card from './Card.svelte';
	import { endTurn, gameState, playCard } from '$lib/socket/socket.svelte';
	import { checkRequirements, isTradeable } from '$lib/shared/lib';
	import { getEntity, isSpellAndHasNoValidTarget, needsTarget } from '$lib/util';
	import { drag } from '$lib/drag.svelte';

	let handElement: HTMLElement;

	const CARD_CX = 85;
	const CARD_CY = 125;

	let hoverIndex: number | null = $state(null);
	let returning = $state(false);
	let dragging = $derived(drag.index !== null && !returning);

	const fanStyle = (i: number, total: number) => {
		const offset = i - (total - 1) / 2;
		const spacing = Math.min(70, 400 / Math.max(total, 1));
		const rot = offset * 5;
		const arc = offset * offset * 2;

		let spread = 0;
		const activeIdx = drag.index ?? hoverIndex;
		if (activeIdx !== null && i !== activeIdx) {
			spread = i < activeIdx ? -45 : 45;
		}

		return `--fan-x: ${offset * spacing + spread}px; --fan-rot: ${rot}deg; --fan-arc: ${arc}px`;
	};

	const setHover = (index: number) => {
		if (!dragging) hoverIndex = index;
	};
	const clearHover = () => {
		if (!dragging) hoverIndex = null;
	};

	const beginDrag = (index: number, event: MouseEvent) => {
		const raw = gameState.self_board.hand[index];
		if (!raw) return;
		const card = getEntity(raw);

		// don't allow dragging unaffordable cards
		if (card.cost > gameState.self_board.mana && !isTradeable(card)) return;

		// spell requires target that is not present dont drag around (unless tradeable)
		if (isSpellAndHasNoValidTarget(card, gameState) && !isTradeable(card)) return;

		// if you dont have that 1 mana neccesary to trade the card
		if (isTradeable(card) && gameState.self_board.mana === 0) return;

		event.preventDefault();

		const cardEl = handElement
			.querySelectorAll('.card-container')
			[index]?.querySelector('.hand-card');
		const rect = cardEl?.getBoundingClientRect();

		drag.index = index;
		drag.x = rect ? rect.left : event.clientX - CARD_CX;
		drag.y = rect ? rect.top : event.clientY - CARD_CY;
		drag.consumed = false;
		hoverIndex = null;
	};

	const endDrag = async () => {
		if (drag.index === null) return;

		const idx = drag.index;

		// a drop zone consumed the drag — just clean up
		if (drag.consumed) {
			drag.consumed = false;
			drag.index = null;
			return;
		}

		if (!handElement) return;
		const handRect = handElement.getBoundingClientRect();
		const cx = drag.x + CARD_CX;
		const cy = drag.y + CARD_CY;
		const isWithinHand =
			cx >= handRect.left && cx <= handRect.right && cy >= handRect.top && cy <= handRect.bottom;

		if (!isWithinHand) {
			// dropped outside but no drop zone caught it
			// only play directly if no targeting needed
			const raw = gameState.self_board.hand[idx];
			const card = getEntity(raw);
			if (card) {
				if (!needsTarget(card) || isSpellAndHasNoValidTarget(card, gameState)) {
					playCard({ index: idx });
				}
				// needs a target but wasn't dropped on one --> return to hand
			}
			drag.index = null;
			return;
		}

		// return to hand animation
		returning = true;
		await tick();
		await new Promise((r) => requestAnimationFrame(r));

		const container = handElement.querySelectorAll('.card-container')[idx];
		if (container) {
			const target = container.getBoundingClientRect();
			drag.x = target.left;
			drag.y = target.top;
		}

		setTimeout(() => {
			returning = false;
			drag.index = null;
		}, 250);
	};

	const onMouseMove = (e: { movementX: number; movementY: number }) => {
		if (!dragging) return;
		drag.x += e.movementX;
		drag.y += e.movementY;
	};
</script>

<svelte:window onmouseup={endDrag} onmousemove={onMouseMove} />

<div class="hand" bind:this={handElement}>
	<button class="end" class:inactive={!gameState.your_turn} onclick={() => endTurn()}>
		END TURN
	</button>
	<div class="mana">{gameState.self_board.mana}/{gameState.self_board.base_mana}</div>
	{#each gameState.self_board.hand as raw, index}
		{@const card = getEntity(raw)}
		{@const affordable =
			card.cost <= gameState.self_board.mana && !isSpellAndHasNoValidTarget(card, gameState)}
		{@const procced =
			affordable &&
			card.card.abilities?.some(
				(a) =>
					a.requirements &&
					a.requirements.length > 0 &&
					checkRequirements(
						a.requirements,
						gameState.self_board,
						gameState.enemy_board,
						gameState,
						card
					)
			)}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			class:hovered={hoverIndex === index && !dragging}
			style="{fanStyle(index, gameState.self_board.hand.length)}; z-index: {index}"
			onmouseenter={() => setHover(index)}
			onmouseleave={clearHover}
			onmousedown={(e) => {
				if (hoverIndex === index) beginDrag(index, e);
			}}
		>
			<div
				class="hand-card"
				class:dragged-away={drag.index === index}
				class:glow-blue={affordable}
				class:glow-yellow={procced}
			>
				<Card {card} />
			</div>
		</div>
	{/each}
	{#if drag.index !== null && !drag.consumed}
		{@const draggedCard = getEntity(gameState.self_board.hand[drag.index])}
		<div class="dragger" class:returning style="left: {drag.x}px; top: {drag.y}px;">
			<Card card={draggedCard} />
		</div>
	{/if}
</div>

<style lang="scss">
	.mana {
		display: flex;
		justify-content: center;
		align-items: center;
		align-self: flex-start;
		font-size: x-large;
		color: $white;
		height: 45px;
		width: 45px;
		margin: 5px;
		border-radius: 100px;
		background-color: rgb(88, 120, 161);
	}

	.dragger {
		position: fixed;
		z-index: 1000;
		pointer-events: none;
		&.returning {
			transition:
				left 0.25s ease,
				top 0.25s ease,
				scale 0.25s ease,
				opacity 0.25s ease;
			scale: 0.6;
			opacity: 0;
		}
	}

	button.end {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 45px;
		padding: 15px;
		margin: 5px;
		color: $white;
		font-size: large;
		background-color: #61aabe;
		border: none;
		border-radius: 10px;
		&:hover {
			background-color: #5c9c97;
			cursor: pointer;
		}
		&.inactive {
			pointer-events: none;
			background-color: #b7bfbe;
		}
	}

	.hand {
		display: flex;
		justify-content: flex-end;
		margin: 0 auto;
		position: relative;
		height: 100%;
		width: 100%;
	}

	.card-container {
		position: absolute;
		height: 147px;
		width: 170px;
		transition: all 0.3s ease;
		left: 50%;
		transform: translateX(calc(-50% + var(--fan-x))) translateY(calc(10% + var(--fan-arc)))
			rotate(var(--fan-rot));
		transform-origin: center bottom;
	}

	.hand-card {
		scale: 0.6;
		translate: 0 0;
		rotate: 0deg;
		opacity: 1;
		transform-origin: center bottom;
		transition:
			scale 0.2s ease,
			translate 0.2s ease,
			rotate 0.2s ease,
			opacity 0.15s ease;
		&.dragged-away {
			opacity: 0;
		}
	}

	.card-container.hovered .hand-card {
		scale: 1;
		translate: 0 -40%;
		rotate: calc(var(--fan-rot) * -1);
		cursor: pointer;
	}
</style>
