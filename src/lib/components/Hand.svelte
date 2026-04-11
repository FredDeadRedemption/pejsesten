<script lang="ts">
	import { tick } from 'svelte';
	import Card from './Card.svelte';
	import { endTurn, gameState, playCard } from '$lib/socket/socket.svelte';
	import type { CardEntity } from '$lib/shared/types';
	import { beginTargeting } from '$lib/targeting.svelte';
	import { checkRequirement } from '$lib/shared/lib';
	import { isSpellAndHasNoValidTarget } from '$lib/util';
	import { drag } from '$lib/drag.svelte';

	let handElement: HTMLElement;

	const CARD_CX = 85;
	const CARD_CY = 125;

	let hoverIndex: number | null = $state(null);
	let draggerIndex: number | null = $state(null);
	let dragCoords = $state({ x: 0, y: 0 });
	let dragCard: CardEntity | null = $state(null);
	let returning = $state(false);
	let dragging = $derived(draggerIndex !== null && !returning);

	const fanStyle = (i: number, total: number) => {
		const offset = i - (total - 1) / 2;
		const spacing = Math.min(70, 400 / Math.max(total, 1));
		const rot = offset * 5;
		const arc = offset * offset * 2;

		let spread = 0;
		const activeIdx = draggerIndex ?? hoverIndex;
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
		event.preventDefault();

		const cardEl = handElement
			.querySelectorAll('.card-container')
			[index]?.querySelector('.hand-card');
		const rect = cardEl?.getBoundingClientRect();

		draggerIndex = index;
		dragCard = gameState.self.hand[index]!;
		hoverIndex = null;

		dragCoords = rect
			? { x: rect.left, y: rect.top }
			: { x: event.clientX - CARD_CX, y: event.clientY - CARD_CY };

		// sync to shared drag store so battlefield can read it
		drag.card = dragCard;
		drag.index = index;
		drag.consumed = false;
	};

	const endDrag = async () => {
		if (!dragCard || draggerIndex === null) return;

		const idx = draggerIndex;

		// a drop zone consumed the drag — just clean up
		if (drag.consumed) {
			drag.consumed = false;
			drag.card = null;
			drag.index = null;
			dragCard = null;
			draggerIndex = null;
			return;
		}

		if (!handElement) return;
		const handRect = handElement.getBoundingClientRect();
		const cx = dragCoords.x + CARD_CX;
		const cy = dragCoords.y + CARD_CY;
		const isWithinHand =
			cx >= handRect.left && cx <= handRect.right && cy >= handRect.top && cy <= handRect.bottom;

		if (!isWithinHand) {
			// dropped outside but no drop zone caught it
			// only play directly if no targeting needed
			const card = gameState.self.hand[idx];
			if (card) {
				const needsTarget = card.abilities.some(
					(a) =>
						a.trigger === 'onPlay' &&
						a.effects.some((e) => 'targetSpec' in e && e.targetSpec.scope === 'single')
				);
				if (!needsTarget || isSpellAndHasNoValidTarget(card, gameState)) {
					playCard({ index: idx });
				}
				// needs a target but wasn't dropped on one → falls through to return animation
			}
			drag.card = null;
			drag.index = null;
			dragCard = null;
			draggerIndex = null;
			return;
		}

		// return to hand animation
		returning = true;
		await tick();
		await new Promise((r) => requestAnimationFrame(r));

		const container = handElement.querySelectorAll('.card-container')[idx];
		if (container) {
			const target = container.getBoundingClientRect();
			dragCoords = { x: target.left, y: target.top };
		}

		setTimeout(() => {
			returning = false;
			drag.card = null;
			drag.index = null;
			dragCard = null;
			draggerIndex = null;
		}, 250);
	};

	const onMouseMove = (e: { movementX: number; movementY: number }) => {
		if (!dragging) return;
		dragCoords.x += e.movementX;
		dragCoords.y += e.movementY;
		drag.x = dragCoords.x;
		drag.y = dragCoords.y;
	};
</script>

<svelte:window onmouseup={endDrag} onmousemove={onMouseMove} />

<div class="hand" bind:this={handElement} class:enemy={!self}>
	{#if self}
		<button class="end" class:inactive={!gameState.yourTurn} onclick={() => endTurn()}>
			END TURN
		</button>
	{/if}
	<div class="mana">{gameState.self.mana}/{gameState.self.baseMana}</div>
	{#each gameState.self.hand as card, index}
		{@const affordable =
			card.cost <= gameState.self.mana && !isSpellAndHasNoValidTarget(card, gameState)}
		{@const procced =
			affordable &&
			card.abilities?.some(
				(a) =>
					a.requirements &&
					a.requirements.length > 0 &&
					a.requirements.every((r) =>
						checkRequirement(r, gameState.self, gameState.enemy, gameState)
					)
			)}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			class:hovered={hoverIndex === index && !dragging}
			style="{fanStyle(index, gameState.self.hand.length)}; z-index: {index}"
			onmouseenter={() => setHover(index)}
			onmouseleave={clearHover}
			onmousedown={(e) => {
				if (hoverIndex === index) beginDrag(index, e);
			}}
		>
			<div
				class="hand-card"
				class:dragged-away={draggerIndex === index}
				class:glow-blue={affordable}
				class:glow-yellow={procced}
			>
				<Card {card} />
			</div>
		</div>
	{/each}
	<!-- hide dragger when battlefield has consumed it (targeting mode active) -->
	{#if dragCard && !drag.consumed}
		<div class="dragger" class:returning style="left: {dragCoords.x}px; top: {dragCoords.y}px;">
			<Card card={dragCard} />
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
		&.enemy {
			transform: translateY(-150px);
		}
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