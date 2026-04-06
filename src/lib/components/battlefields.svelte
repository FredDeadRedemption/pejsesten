<script lang="ts">
	import CardSmall from './cardSmall.svelte';
	import { attack, playCard } from '$lib/socket/socket';
	import type { MinionEntity } from '$lib/shared/types';
	import { endTargeting, targeting } from '$lib/targeting.svelte';

	let {
		selfBattleField = $bindable(),
		enemyBattleField = $bindable(),
		selfHP = $bindable(),
		enemyHP = $bindable()
	}: {
		selfBattleField: MinionEntity[];
		enemyBattleField: MinionEntity[];
		selfHP: number;
		enemyHP: number;
	} = $props();

	let mouseX = $state(0);
	let mouseY = $state(0);
	let originRect = $state<DOMRect | null>(null);
	let origin: number | null = $state(null);

	const handleMouseMove = (e: MouseEvent) => {
		mouseX = e.clientX;
		mouseY = e.clientY;
	};

	const beginAttack = (index: number, e: MouseEvent) => {
		origin = index;
		const el = e.currentTarget as HTMLElement;
		originRect = el.getBoundingClientRect();
	};

	const cancelAttack = () => (origin = null);

	const tryAttack = (index: number, face: boolean) => {
		if (origin === null) return;
		attack({
			origin: origin,
			target: index,
			face: face
		});
		origin = null;
	};

	let didFireTargeting = $state(false);

	const handleTargetInteraction = (e: MouseEvent, index: number) => {
		e.stopPropagation();
		if (!targeting.active) return;
		if (didFireTargeting) {
			didFireTargeting = false;
			return;
		}
		didFireTargeting = true;
		console.log('did fire targeting', didFireTargeting, targeting.cardIndex, index);
		playCard({ index: targeting.cardIndex!, target: index });
		endTargeting();
		setTimeout(() => (didFireTargeting = false), 50);
	};

	const handleFriendlyTargetInteraction = (e: MouseEvent, index: number) => {
		e.stopPropagation();
		if (!targeting.active) return;
		if (didFireTargeting) {
			didFireTargeting = false;
			return;
		}
		didFireTargeting = true;
		playCard({ index: targeting.cardIndex!, target: index });
		endTargeting();
		setTimeout(() => (didFireTargeting = false), 50);
	};
</script>

<svelte:window onclick={cancelAttack} onmousemove={handleMouseMove} />

{#if origin !== null && originRect}
	{@const x1 = originRect.left + originRect.width / 2}
	{@const y1 = originRect.top + originRect.height / 2}
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

<div class="enemy-battlefield">
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="hero enemy"
		onclick={(e) => {
			e.stopPropagation(); // so it doesnt also trigger cancelAttack prevent event bubbling
			tryAttack(-1, true);
		}}
	>
		<span class="hp">{enemyHP}</span>
	</div>
	{#each enemyBattleField as card, index (`${card.id}-${index}`)}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			style="--i: {index}; --total: {enemyBattleField.length}"
			onmouseenter={() => {
				if (targeting.active) targeting.hoveredTarget = index;
			}}
			onmouseleave={() => {
				if (targeting.active) targeting.hoveredTarget = null;
			}}
			onmouseup={(e) => {
				if (targeting.active) handleTargetInteraction(e, index);
				else tryAttack(index, false);
			}}
			onclick={(e) => {
				if (targeting.active) handleTargetInteraction(e, index);
				else tryAttack(index, false);
			}}
		>
			<CardSmall {card}></CardSmall>
		</div>
	{/each}
</div>
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="self-battlefield" onclick={cancelAttack}>
	<div class="hero self"><span class="hp">{selfHP}</span></div>
	{#each selfBattleField as card, index (`${card.id}-${index}`)}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="card-container"
			style="--i: {index}; --total: {selfBattleField.length}"
			class:selected={origin === index}
			onmouseenter={() => {
				if (targeting.active) targeting.hoveredTarget = index;
			}}
			onmouseleave={() => {
				if (targeting.active) targeting.hoveredTarget = null;
			}}
			onmouseup={(e) => {
				e.stopPropagation();
				if (targeting.active) {
					handleFriendlyTargetInteraction(e, index);
				} else {
					beginAttack(index, e);
				}
			}}
			onclick={(e) => {
				e.stopPropagation();
				if (targeting.active) handleFriendlyTargetInteraction(e, index);
				else beginAttack(index, e);
			}}
		>
			<CardSmall {card}></CardSmall>
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

		/* Centered overlapping translation */
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
		border-radius: 100px;
		&.self {
			align-self: flex-end;
			transform: translateY(30px);
			background-color: rgb(102, 102, 174);
		}
		&.enemy {
			align-self: flex-start;
			transform: translateY(-30px);
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
