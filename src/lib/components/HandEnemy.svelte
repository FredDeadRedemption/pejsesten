<script lang="ts">
	import Card from './Card.svelte';
	import { gameState } from '$lib/socket/socket.svelte';
	import type { CardEntity } from '$lib/shared/types';
	import { checkRequirement } from '$lib/shared/lib';
	import { isSpellAndHasNoValidTarget } from '$lib/util';

	let handElement: HTMLElement;

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

  // TODO: send hover event over seperate socket channel?
	const setHover = (index: number) => {
		if (!dragging) hoverIndex = index;
	};
	const clearHover = () => {
		if (!dragging) hoverIndex = null;
	};
</script>

<div class="hand" bind:this={handElement} class:enemy={!self}>
	<div class="mana" class:self>{gameState.enemy.mana}/{gameState.self.baseMana}</div>
	{#each gameState.enemy.hand as card, index}
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
			style="{self
				? fanStyle(index, gameState.enemy.hand.length)
				: `--fan-x: ${(index - (gameState.enemy.hand.length - 1) / 2) * 40}px; --fan-rot: 0deg; --fan-arc: 0px`}; z-index: {index}"
		>
			<div
				class="hand-card"
				class:dragged-away={draggerIndex === index}
				class:affordable
				class:procced
			>
				<Card {card} />
			</div>
		</div>
	{/each}
	{#if dragCard}
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
			transition:
				left 0.25s ease,
				top 0.25s ease,
				scale 0.25s ease,
				opacity 0.25s ease;
			scale: 0.6;
			opacity: 0;
		}
	}

	.hand {
		display: flex;
		justify-content: flex-end;
		margin: 0 auto;
		width: fit-content;
		position: relative;
		height: 100%;
		width: 100%;
		transform: translateY(-150px);
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
