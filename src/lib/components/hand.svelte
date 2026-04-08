<script lang="ts">
	import { tick } from 'svelte';
	import Card from './card.svelte';
	import { endTurn, gameState, playCard } from '$lib/socket/socket.svelte';
	import type { CardEntity } from '$lib/shared/types';
	import { beginTargeting } from '$lib/targeting.svelte';
	import { checkRequirement } from '$lib/shared/lib';
	import { isSpellAndHasNoValidTarget } from '$lib/util';

	let handElement: HTMLElement;

	let {
		hand = $bindable(),
		mana = $bindable(),
		self = false,
		yourTurn = false
	}: { hand: CardEntity[]; mana: number; self?: boolean; yourTurn?: boolean } = $props();

	// half card dimensions, used to find card center from top-left coords
	const CARD_CX = 85;
	const CARD_CY = 125;

	let hoverIndex: number | null = $state(null);
	let draggerIndex: number | null = $state(null);
	let dragCoords = $state({ x: 0, y: 0 });
	let dragCard: CardEntity | null = $state(null);
	let returning = $state(false);
	let dragging = $derived(draggerIndex !== null && !returning);

	// fan layout: rotation, arc, horizontal spread, and neighbor displacement
	const fanStyle = (i: number, total: number) => {
		const offset = i - (total - 1) / 2;
		const spacing = Math.min(70, 400 / Math.max(total, 1));
		const rot = offset * 5;
		const arc = offset * offset * 2;

		// keep neighbors spread while hovering or dragging
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

		// capture card's screen position before state changes
		const cardEl = handElement.querySelectorAll('.card-container')[index]?.querySelector('.hand-card');
		const rect = cardEl?.getBoundingClientRect();

		draggerIndex = index;
		dragCard = hand[index]!;
		hoverIndex = null;

		dragCoords = rect
			? { x: rect.left, y: rect.top }
			: { x: event.clientX - CARD_CX, y: event.clientY - CARD_CY };
	};

	const endDrag = async () => {
		if (!dragCard || draggerIndex === null) return;

		const idx = draggerIndex;

		if (!handElement) return;
		const handRect = handElement.getBoundingClientRect();
		const cx = dragCoords.x + CARD_CX;
		const cy = dragCoords.y + CARD_CY;
		const isWithinHand =
			cx >= handRect.left && cx <= handRect.right &&
			cy >= handRect.top && cy <= handRect.bottom;

		if (!isWithinHand) {
			// play the card
			const card = hand[idx];
			if (card) {
				const needsTarget = card.abilities.some(
					(a) =>
						a.trigger === 'onPlay' &&
						a.effects.some((e) => 'targetSpec' in e && e.targetSpec.scope === 'single')
				);
				if (needsTarget && !isSpellAndHasNoValidTarget(card, gameState)) {
					beginTargeting(card, idx);
				} else {
					playCard({ index: idx });
				}
			}
			dragCard = null;
			draggerIndex = null;
			return;
		}

		// return to hand: animate dragger back to fan position
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
			dragCard = null;
			draggerIndex = null;
		}, 250);
	};

	const onMouseMove = (e: { movementX: number; movementY: number }) => {
		if (!dragging) return;
		dragCoords.x += e.movementX;
		dragCoords.y += e.movementY;
	};
</script>

<svelte:window onmouseup={endDrag} onmousemove={onMouseMove} />

<div id="hand" bind:this={handElement}>
	{#if self}
		<button class="end" class:self class:inactive={!yourTurn} onclick={() => endTurn()}
			>END TURN</button
		>
	{/if}
	<div class="mana" class:self>{mana}/{gameState.self.baseMana}</div>
	{#each hand as card, index}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			class:hovered={hoverIndex === index && !dragging}
			style="{self ? fanStyle(index, hand.length) : `--fan-x: ${(index - (hand.length - 1) / 2) * 40}px; --fan-rot: 0deg; --fan-arc: 0px`}; z-index: {index}"
			onmouseenter={() => setHover(index)}
			onmouseleave={clearHover}
			onmousedown={(e) => { if (hoverIndex === index) beginDrag(index, e) }}
		>
			<div
				class="hand-card"
				class:dragged-away={draggerIndex === index}
				class:affordable={card.cost <= gameState.self.mana &&
					!isSpellAndHasNoValidTarget(card, gameState)}
				class:procced={!isSpellAndHasNoValidTarget(card, gameState) &&
					card.cost <= gameState.self.mana &&
					card.abilities?.some(
						(a) =>
							a.requirements &&
							a.requirements.length > 0 &&
							a.requirements.every((r) =>
								checkRequirement(r, gameState.self, gameState.enemy, gameState)
							)
					)}
			>
				<Card {card} />
			</div>
		</div>
	{/each}
	{#if dragCard}
		<div
			class="dragger"
			class:returning
			style="left: {dragCoords.x}px; top: {dragCoords.y}px;"
		>
			<Card card={dragCard} />
		</div>
	{/if}
</div>

<style lang="scss">
	.mana {
		display: flex;
		justify-content: center;
		align-items: center;
		align-self: flex-end;
		font-size: x-large;
		color: $white;
		height: 45px;
		width: 45px;
		margin: 5px;
		border-radius: 100px;
		background-color: rgb(88, 120, 161);
		&.self {
			align-self: flex-start;
		}
	}

	.dragger {
		position: fixed;
		z-index: 1000;
		pointer-events: none;
		&.returning {
			transition: left 0.25s ease, top 0.25s ease, scale 0.25s ease, opacity 0.25s ease;
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

	#hand {
		display: flex;
		justify-content: flex-end;
		margin: 0 auto;
		width: fit-content;
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
		transform: translateX(calc(-50% + var(--fan-x)))
			translateY(calc(10% + var(--fan-arc)))
			rotate(var(--fan-rot));
		transform-origin: center bottom;
	}

	.hand-card {
		scale: 0.6;
		translate: 0 0;
		rotate: 0deg;
		opacity: 1;
		transform-origin: center bottom;
		transition: scale 0.2s ease, translate 0.2s ease, rotate 0.2s ease, opacity 0.15s ease;
		&.dragged-away {
			opacity: 0;
		}
	}
	.card-container.hovered .hand-card {
		scale: 1;
		translate: 0 -60%;
		rotate: calc(var(--fan-rot) * -1);
		cursor: pointer;
	}
</style>
