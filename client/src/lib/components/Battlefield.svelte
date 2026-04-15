<script lang="ts">
	import CardSmall from './CardSmall.svelte';
	import { attack, gameState, playCard } from '$lib/socket/socket.svelte';
	import { endTargeting, targeting, beginTargeting } from '$lib/targeting.svelte';
	import { isSpellAndHasNoValidTarget } from '$lib/util';
	import { drag, resetDrag } from '$lib/drag.svelte';

	let mouseX = $state(0);
	let mouseY = $state(0);

	let attackOrigin: string | null = $state(null);
	let attackOriginRect = $state<DOMRect | null>(null);

	const handleMouseMove = (e: MouseEvent) => {
		mouseX = e.clientX;
		mouseY = e.clientY;
	};

	const beginAttack = (entityID: string, e: MouseEvent) => {
		e.preventDefault();
		attackOrigin = entityID;
		const el = e.currentTarget as HTMLElement;
		attackOriginRect = el.getBoundingClientRect();
	};

	const cancelAttack = () => {
		attackOrigin = null;
		attackOriginRect = null;
	};

	const endAttack = (entityID: string) => {
		if (attackOrigin === null) return;
		attack({ originID: attackOrigin, targetID: entityID });
		attackOrigin = null;
		attackOriginRect = null;
	};

	// when a dragged card enters the battlefield area, check if it needs targeting
	// if so: switch from drag mode to targeting mode immediately
	const onBattlefieldEnter = () => {
		if (!drag.card || drag.index === null) return;
		const card = drag.card;
		const idx = drag.index;

		const needsTarget = card.abilities.some(
			(a) =>
				a.trigger === 'onPlay' &&
				a.effects.some((e) => 'targetSpec' in e && e.targetSpec.scope === 'single')
		);

		if (needsTarget && !isSpellAndHasNoValidTarget(card, gameState)) {
			// switch to targeting mode — hide the dragger, show targeting arrow
			drag.consumed = true;
			beginTargeting(card, idx);
		}
	};

	// dropping a non-targeting card onto the battlefield plays it
	const onBattlefieldDrop = () => {
		if (!drag.card || drag.index === null) return;
		if (drag.consumed) return; // already handled (targeting mode)

		const card = drag.card;
		const idx = drag.index;

		const needsTarget = card.abilities.some(
			(a) =>
				a.trigger === 'onPlay' &&
				a.effects.some((e) => 'targetSpec' in e && e.targetSpec.scope === 'single')
		);

		// only play directly if no target needed or no valid targets exist
		if (!needsTarget || isSpellAndHasNoValidTarget(card, gameState)) {
			drag.consumed = true;
			playCard({ index: idx });
		}
	};

	let didFireTargeting = $state(false);

	const handleTargetInteraction = (e: MouseEvent, entityID: string) => {
		e.stopPropagation();
		if (!targeting.active) return;
		if (didFireTargeting) {
			didFireTargeting = false;
			return;
		}
		didFireTargeting = true;
		playCard({ index: targeting.cardIndex!, target: entityID });
		endTargeting();
		resetDrag();
		setTimeout(() => (didFireTargeting = false), 50);
	};
</script>

<svelte:window onclick={cancelAttack} onmousemove={handleMouseMove} />

{#if attackOrigin !== null && attackOriginRect}
	{@const x1 = attackOriginRect.left + attackOriginRect.width / 2}
	{@const y1 = attackOriginRect.top + attackOriginRect.height / 2}
	{@const dx = mouseX - x1}
	{@const dy = mouseY - y1}
	{@const dist = Math.hypot(dx, dy)}
	{@const bend = dist * 0.1}
	{@const cx = (x1 + mouseX) / 2 - (dy / dist) * bend}
	{@const cy = (y1 + mouseY) / 2 + (dx / dist) * bend}
	<svg class="attack-line">
		<defs>
			<linearGradient
				id="arrow-gradient"
				gradientUnits="userSpaceOnUse"
				{x1}
				{y1}
				x2={mouseX}
				y2={mouseY}
			>
				<stop offset="0%" stop-color="#3B130C" />
				<stop offset="100%" stop-color="#E05236" />
			</linearGradient>
			<marker id="arrow" markerWidth="6" markerHeight="6" refX="3" refY="3" orient="auto">
				<path d="M0,0 L0,6 L6,3 z" fill="#E05236" />
			</marker>
		</defs>
		<path
			d="M {x1} {y1} Q {cx} {cy} {mouseX} {mouseY}"
			stroke="url(#arrow-gradient)"
			stroke-width="2"
			stroke-dasharray="6,3"
			fill="none"
			marker-end="url(#arrow)"
		/>
	</svg>
{/if}

<!-- enemy battlefield -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="enemy-battlefield" onmouseenter={onBattlefieldEnter} onmouseup={onBattlefieldDrop}>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="hero enemy"
		onclick={(e) => {
			e.stopPropagation();
			endAttack('heroEnemy');
		}}
		onmouseup={(e) => {
			if (targeting.active) handleTargetInteraction(e, 'heroEnemy');
			else endAttack('heroEnemy');
		}}
	>
		<span class="hp">{gameState.enemy.hero.defence}</span>
	</div>

	{#each gameState.enemy.battlefield as card, index (`${card.id}-${index}`)}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			style="--i: {index}; --total: {gameState.enemy.battlefield.length}"
			onmouseenter={() => {
				if (targeting.active) targeting.hoveredTarget = card.entityID;
			}}
			onmouseleave={() => {
				if (targeting.active) targeting.hoveredTarget = null;
			}}
			onmouseup={(e) => {
				if (targeting.active) handleTargetInteraction(e, card.entityID);
				else if (attackOrigin) endAttack(card.entityID);
				else endAttack(card.entityID);
			}}
			onclick={(e) => {
				if (targeting.active) handleTargetInteraction(e, card.entityID);
				else endAttack(card.entityID);
			}}
		>
			<CardSmall {card} />
		</div>
	{/each}
</div>

<!-- self battlefield -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="self-battlefield"
	onclick={cancelAttack}
	onmouseenter={onBattlefieldEnter}
	onmouseup={onBattlefieldDrop}
>
	<div
		class="hero self"
		onmouseup={(e) => {
			if (targeting.active) handleTargetInteraction(e, 'heroSelf');
		}}
	>
		<span class="hp">{gameState.self.hero.defence}</span>
	</div>

	{#each gameState.self.battlefield as card, index (`${card.id}-${index}`)}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			style="--i: {index}; --total: {gameState.self.battlefield.length}"
			class:selected={attackOrigin === card.entityID}
			onmouseenter={() => {
				if (targeting.active) targeting.hoveredTarget = card.entityID;
			}}
			onmouseleave={() => {
				if (targeting.active) targeting.hoveredTarget = null;
			}}
			onmousedown={(e) => {
				e.stopPropagation();
				if (!targeting.active) beginAttack(card.entityID, e);
			}}
			onmouseup={(e) => {
				e.stopPropagation();
				if (targeting.active) handleTargetInteraction(e, card.entityID);
			}}
			onclick={(e) => {
				e.stopPropagation();
				if (targeting.active) handleTargetInteraction(e, card.entityID);
				else beginAttack(card.entityID, e);
			}}
		>
			<CardSmall {card} />
		</div>
	{/each}
</div>

<style lang="scss">
	.hp {
		user-select: none;
		color: white;
		font-size: 1.5rem;
	}

	.self-battlefield,
	.enemy-battlefield {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.card-container {
		position: absolute;
		height: 147px;
		width: 100px;
		left: 50%;
		transform: translateX(calc(-50% + (var(--i) - (var(--total) - 1) / 2) * 110px));
		&.selected {
			outline: 2px solid $secondary !important;
			border-radius: 4px;
		}
	}

	.hero {
		display: flex;
		justify-content: center;
		align-items: center;
		position: absolute;
		height: 60px;
		width: 60px;
		&.self {
			align-self: flex-end;
			transform: translateY(60px);
			background-color: rgb(102, 102, 174);
		}
		&.enemy {
			align-self: flex-start;
			transform: translateY(-60px);
			background-color: rgb(192, 86, 86);
		}
	}

	.attack-line {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		pointer-events: none;
		z-index: 999;
	}
</style>
