<script lang="ts">
	import CardSmall from './CardSmall.svelte';
	import { attack, gameState, playCard } from '$lib/socket.svelte';
	import { endTargeting, targeting, beginTargeting } from '$lib/targeting.svelte';
	import { getEntity, isSpellAndHasNoValidTarget, hasNoValidTarget, needsTarget } from '$lib/lib';
	import { drag, resetDrag } from '$lib/drag.svelte';

	let mouseX = $state(0);
	let mouseY = $state(0);

	let attackOrigin: number | null = $state(null);
	let attackOriginRect = $state<DOMRect | null>(null);
	let attackOriginEl = $state<HTMLElement | null>(null);
	let animating = $state(false);

	const handleMouseMove = (e: MouseEvent) => {
		mouseX = e.clientX;
		mouseY = e.clientY;
	};

	const beginAttack = (entity_id: number, e: MouseEvent) => {
		e.preventDefault();
		attackOrigin = entity_id;
		attackOriginEl = e.currentTarget as HTMLElement;
		attackOriginRect = attackOriginEl.getBoundingClientRect();
	};

	const cancelAttack = () => {
		if (animating) return;
		attackOrigin = null;
		attackOriginRect = null;
		attackOriginEl = null;
	};

	const endAttack = async (entity_id: number) => {
		if (attackOrigin === null || animating) return;

		const anyGuard = gameState.enemy_board.battlefield.some((m) =>
			m.card.attributes.includes('Guard')
		);
		if (anyGuard) {
			const targetMinion = gameState.enemy_board.battlefield.find((m) => m.entity_id === entity_id);
			if (!targetMinion?.card.attributes.includes('Guard')) {
				cancelAttack();
				return;
			}
		}

		const originId = attackOrigin;
		const originEl = attackOriginEl;
		animating = true;

		if (originEl) {
			const targetEl = document.querySelector(
				`[data-entity-id="${entity_id}"]`
			) as HTMLElement | null;
			const fromRect = originEl.getBoundingClientRect();
			const toRect = targetEl?.getBoundingClientRect();
			if (toRect) {
				const dx = toRect.left + toRect.width / 2 - (fromRect.left + fromRect.width / 2);
				const dy = toRect.top + toRect.height / 2 - (fromRect.top + fromRect.height / 2);
				originEl.style.transition = 'translate 70ms cubic-bezier(0.1, 0, 0.2, 1)';
				originEl.style.translate = `${dx * 0.7}px ${dy * 0.7}px`;
				await new Promise((r) => setTimeout(r, 70));
				originEl.style.transition = 'translate 120ms cubic-bezier(0.5, 0, 0.8, 0.5)';
				originEl.style.translate = '';
				await new Promise((r) => setTimeout(r, 120));
				originEl.style.transition = '';
			}
		}

		attack({ origin_id: originId, target_id: entity_id });
		attackOrigin = null;
		attackOriginRect = null;
		attackOriginEl = null;
		animating = false;
	};

	// when a dragged card enters the battlefield area, check if it needs targeting
	// if so: switch from drag mode to targeting mode immediately
	const onBattlefieldEnter = () => {
		if (drag.index === null) return;
		const raw = gameState.self_board.hand[drag.index];
		if (!raw) return;
		const card = getEntity(raw);

		if (needsTarget(card, gameState) && !hasNoValidTarget(card, gameState)) {
			// switch to targeting mode — hide the dragger, show targeting arrow
			drag.consumed = true;
			beginTargeting(card, drag.index);
		}
	};

	// dropping a non-targeting card onto the battlefield plays it
	const onBattlefieldDrop = () => {
		if (drag.index === null) return;
		if (drag.consumed) return; // already handled (targeting mode)
		if (drag.index === null) return;
		const raw = gameState.self_board.hand[drag.index];
		if (!raw) return;
		const card = getEntity(raw);

		// only play directly if no target needed or no valid targets exist
		if (!needsTarget(card, gameState) || hasNoValidTarget(card, gameState)) {
			drag.consumed = true;
			playCard({ index: drag.index });
		}
	};

	let didFireTargeting = $state(false);

	const handleTargetInteraction = (e: MouseEvent, entity_id: number) => {
		e.stopPropagation();
		if (!targeting.active) return;
		if (didFireTargeting) {
			didFireTargeting = false;
			return;
		}
		didFireTargeting = true;
		playCard({ index: targeting.cardIndex!, target: entity_id });
		endTargeting();
		resetDrag();
		setTimeout(() => (didFireTargeting = false), 50);
	};

	let enemyHeroTargetable = $derived(
		targeting.active ||
			(attackOrigin != null &&
				!gameState.enemy_board.battlefield.some((m) => m.card.attributes.includes('Guard')))
	);
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
		class:glow-white={enemyHeroTargetable}
		class="hero enemy"
		data-entity-id={gameState.enemy_board.hero.entity_id}
		onclick={(e) => {
			e.stopPropagation();
			endAttack(gameState.enemy_board.hero.entity_id);
		}}
		onmouseup={(e) => {
			if (targeting.active) handleTargetInteraction(e, gameState.enemy_board.hero.entity_id);
			else endAttack(gameState.enemy_board.hero.entity_id);
		}}
	>
		<span class="hp">{gameState.enemy_board.hero.defence}</span>
	</div>
	{#each gameState.enemy_board.battlefield as card, index (`${card.card.id}-${index}`)}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		{@const targetable =
			targeting.active ||
			(attackOrigin != null &&
				(!gameState.enemy_board.battlefield.some((m) => m.card.attributes.includes('Guard')) ||
					card.card.attributes.includes('Guard')))}
		<div
			class:glow-white={targetable}
			class="card-container"
			data-entity-id={card.entity_id}
			style="--i: {index}; --total: {gameState.enemy_board.battlefield.length}"
			onmouseenter={() => {
				if (targeting.active) targeting.hoveredTarget = card.entity_id;
			}}
			onmouseleave={() => {
				if (targeting.active) targeting.hoveredTarget = null;
			}}
			onmouseup={(e) => {
				if (targeting.active) handleTargetInteraction(e, card.entity_id);
				else if (attackOrigin) endAttack(card.entity_id);
				else endAttack(card.entity_id);
			}}
			onclick={(e) => {
				if (targeting.active) handleTargetInteraction(e, card.entity_id);
				else endAttack(card.entity_id);
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
			if (targeting.active) handleTargetInteraction(e, gameState.self_board.hero.entity_id);
		}}
	>
		<span class="hp">{gameState.self_board.hero.defence}</span>
	</div>

	{#each gameState.self_board.battlefield as card, index (`${card.card.id}-${index}`)}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			style="--i: {index}; --total: {gameState.self_board.battlefield.length}"
			class:glow-red={attackOrigin === card.entity_id}
			onmouseenter={() => {
				if (targeting.active) targeting.hoveredTarget = card.entity_id;
			}}
			onmouseleave={() => {
				if (targeting.active) targeting.hoveredTarget = null;
			}}
			onmousedown={(e) => {
				e.stopPropagation();
				if (!targeting.active && !card.exhausted) beginAttack(card.entity_id, e);
			}}
			onmouseup={(e) => {
				e.stopPropagation();
				if (targeting.active) handleTargetInteraction(e, card.entity_id);
			}}
			onclick={(e) => {
				e.stopPropagation();
				if (targeting.active) handleTargetInteraction(e, card.entity_id);
				else if (!card.exhausted) beginAttack(card.entity_id, e);
			}}
		>
			<CardSmall {card} />
		</div>
	{/each}
</div>

<style lang="scss">
	@use '../../vars' as *;
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
