<script lang="ts">
	import { scale } from 'svelte/transition';
	import Card from './card.svelte';
	import { endTurn, gameState, playCard } from '$lib/socket/socket.svelte';
	import type { CardEntity } from '$lib/shared/types';
	import { beginTargeting } from '$lib/targeting.svelte';

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
	function tryPlaceCard(x: number, y: number, dragIndex: number) {
		if (!handElement) return;
		if (dragIndex === null) return;
		const rect = handElement.getBoundingClientRect();
		const cardWidth = 100;
		const cardHeight = 147;
		const isWithinHand =
			x + cardWidth / 2 >= rect.left &&
			x + cardWidth / 2 <= rect.right &&
			y + cardHeight / 2 >= rect.top &&
			y + cardHeight / 2 <= rect.bottom;
		if (isWithinHand) return;

		const card = hand[dragIndex];
		if (!card) return;

		// check if minion needs targeting on play
		// and that target is a single target
		const needsTarget = card.abilities.some(
			(a) =>
				a.trigger === 'onPlay' &&
				a.effects.some((e) => 'targetSpec' in e && e.targetSpec.scope === 'single')
		);

		if (needsTarget) {
			// check if it needs a friendly minion but board is empty
			const needsFriendlyMinion = card.abilities.some(
				(a) =>
					a.trigger === 'onPlay' &&
					a.effects.some(
						(e) =>
							'targetSpec' in e &&
							e.targetSpec.scope === 'single' &&
							e.targetSpec.side === 'friendly' &&
							e.targetSpec.entityType === 'minion'
					)
			);

			const needsEnemyMinion = card.abilities.some(
				(a) =>
					a.trigger === 'onPlay' &&
					a.effects.some(
						(e) =>
							'targetSpec' in e &&
							e.targetSpec.scope === 'single' &&
							e.targetSpec.side === 'enemy' &&
							e.targetSpec.entityType === 'minion'
					)
			);

			// if friendly board is empty and spell needs friendly minion target, just play without target
			if (
				(needsFriendlyMinion && gameState.self.battlefield.length === 0) ||
				(needsEnemyMinion && gameState.enemy.battlefield.length === 0)
			) {
				console.log('NFM ' + needsFriendlyMinion);
				playCard({ index: dragIndex });
				return;
			}

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
	<div class="mana" class:self>{mana}</div>
	{#each hand as card, index}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			style="--i: {index}; --total: {hand.length}"
			onmouseenter={() => setHover(index)}
			onmouseleave={clearHover}
		>
			{#if hoverIndex === index && !draggin}
				<div
					class="hover-card"
					class:affordable={card.cost <= gameState.self.mana}
					onmousedown={(e: MouseEvent) => beginDrag(index, e)}
					in:scale={{ start: 0.9, duration: 250 }}
					out:scale={{ duration: 200 }}
				>
					<Card card={card!} />
				</div>
			{:else if draggerIndex !== index}
				<div class="default-card" class:affordable={card.cost <= gameState.self.mana}>
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
		width: 130px;
		transition: all 0.3s ease;

		/* Centered overlapping translation */
		left: 50%;
		transform: translateX(calc(-50% + (var(--i) - (var(--total) - 1) / 2) * 80px)) translateY(10%);
	}
	.default-card {
		scale: 0.6;
		&.affordable {
			box-shadow:
				0 0 8px 2px rgba(125, 206, 87, 0.6),
				0 0 20px 4px rgba(135, 92, 143, 0.2);
			animation: glow-pulse 2s ease-in-out infinite alternate;
		}

		@keyframes glow-pulse {
			from {
				box-shadow:
					0 0 8px 2px rgba(146, 206, 87, 0.6),
					0 0 20px 4px rgba(121, 206, 87, 0.2);
			}
			to {
				box-shadow:
					0 0 12px 3px rgba(150, 206, 87, 0.8),
					0 0 28px 6px rgba(141, 206, 87, 0.3);
			}
		}
	}
	.hover-card {
		cursor: pointer;
		border-radius: 5px;
		position: absolute;
		top: 0;
		left: -35px;
		transform: translateY(-60%);
		&.affordable {
			box-shadow:
				0 0 8px 2px rgba(125, 206, 87, 0.6),
				0 0 20px 4px rgba(135, 92, 143, 0.2);
			animation: glow-pulse 2s ease-in-out infinite alternate;
		}

		@keyframes glow-pulse {
			from {
				box-shadow:
					0 0 8px 2px rgba(146, 206, 87, 0.6),
					0 0 20px 4px rgba(121, 206, 87, 0.2);
			}
			to {
				box-shadow:
					0 0 12px 3px rgba(150, 206, 87, 0.8),
					0 0 28px 6px rgba(141, 206, 87, 0.3);
			}
		}
	}
</style>
