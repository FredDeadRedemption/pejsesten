<script lang="ts">
	import { scale } from 'svelte/transition';
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

	let hoverIndex: number | null = $state(null); // keeps track of which index to display big card
	let draggerIndex: number | null = $state(null); // keeps track of which index is to hide because it's being dragged
	let dragCoords = $state({ x: 0, y: 0 });
	let dragCard: CardEntity | null = $state(null);

	// fan layout: rotation, arc, and horizontal spread
	const fanStyle = (i: number, total: number) => {
		const offset = i - (total - 1) / 2;
		const spacing = Math.min(70, 400 / Math.max(total, 1));
		const rot = offset * 5;
		const arc = offset * offset * 2;
		return `--fan-x: ${offset * spacing}px; --fan-rot: ${rot}deg; --fan-arc: ${arc}px`;
	};

	const setHover = (index: number) => (hoverIndex = index);
	const clearHover = () => (hoverIndex = null);

	let draggin: boolean = $state(false);
	$effect(() => {
		console.log(draggin);
	});

	const beginDrag = (index: number, event: MouseEvent) => {
		event.preventDefault();
		draggin = true;
		draggerIndex = index;
		dragCoords = { x: event.clientX - 50, y: event.clientY - 73 };
		dragCard = hand[index]!;
	};
	const endDrag = () => {
		draggin = false;
		if (!dragCard || draggerIndex === null) return; // js moment 2
		tryPlaceCard(dragCoords.x, dragCoords.y, draggerIndex);
		dragCard = null;
		draggerIndex = null;
	};
	const onMouseMove = (e: { movementX: number; movementY: number }) => {
		if (!draggin) return;
		dragCoords.x += e.movementX;
		dragCoords.y += e.movementY;
	};
	const tryPlaceCard = (x: number, y: number, dragIndex: number) => {
		if (!handElement) return;
		const rect = handElement.getBoundingClientRect();
		const isWithinHand =
			x + 50 >= rect.left && x + 50 <= rect.right && y + 73 >= rect.top && y + 73 <= rect.bottom;
		if (isWithinHand) return;

		const card = hand[dragIndex];
		if (!card) return;

		const needsTarget = card.abilities.some(
			(a) =>
				a.trigger === 'onPlay' &&
				a.effects.some((e) => 'targetSpec' in e && e.targetSpec.scope === 'single')
		);

		if (needsTarget && !isSpellAndHasNoValidTarget(card, gameState)) {
			beginTargeting(card, dragIndex);
			return;
		}

		playCard({ index: dragIndex });
	}
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
			style="{self ? fanStyle(index, hand.length) : `--fan-x: ${(index - (hand.length - 1) / 2) * 40}px; --fan-rot: 0deg; --fan-arc: 0px`}; z-index: {hoverIndex === index ? 100 : index}"
			onmouseenter={() => setHover(index)}
			onmouseleave={clearHover}
		>
			{#if hoverIndex === index && !draggin}
				<div
					class="hover-card"
					class:affordable={card.cost <= gameState.self.mana && !isSpellAndHasNoValidTarget(card, gameState)}
					class:procced={!isSpellAndHasNoValidTarget(card, gameState) && card.cost <= gameState.self.mana && card.abilities?.some(
						(a) =>
							a.requirements &&
							a.requirements.length > 0 &&
							a.requirements.every((r) =>
								checkRequirement(r, gameState.self, gameState.enemy, gameState)
							)
					)}
					onmousedown={(e: MouseEvent) => beginDrag(index, e)}
					in:scale={{ start: 0.9, duration: 250 }}
					out:scale={{ duration: 200 }}
				>
					<Card card={card!} />
				</div>
			{:else if draggerIndex !== index}
				<div
					class="default-card"
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
					<Card compact={false} {card} />
				</div>
			{/if}
		</div>
	{/each}
	{#if draggin && dragCard}
		<div
			class="dragger"
			style="position: abosolute; left: {dragCoords.x}px; top: {dragCoords.y}px;"
		>
			<Card compact={true} card={dragCard}></Card>
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
		position: fixed; /* Use fixed for smooth dragging */
		z-index: 1000;
		pointer-events: none;
		cursor: grabbing;
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
	.default-card {
		scale: 0.6;
	}
	.hover-card {
		cursor: pointer;
		border-radius: 5px;
		position: absolute;
		top: 0;
		left: -35px;
		transform: translateY(-60%) rotate(calc(var(--fan-rot) * -1));
	}
</style>
