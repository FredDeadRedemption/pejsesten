<script lang="ts">
	import CardSmall from './cardSmall.svelte';
	import { attack } from '$lib/socket/socket';
	import type { MinionEntity } from '$lib/shared/types';

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
</script>

<svelte:window onclick={cancelAttack} onmousemove={handleMouseMove} />

{#if origin !== null && originRect}
	<svg class="attack-line">
		<defs>
			<marker id="arrow" markerWidth="6" markerHeight="6" refX="3" refY="3" orient="auto">
				<path d="M0,0 L0,6 L6,3 z" fill="red" />
			</marker>
		</defs>
		<line
			x1={originRect.left + originRect.width / 2}
			y1={originRect.top + originRect.height / 2}
			x2={mouseX}
			y2={mouseY}
			stroke="red"
			stroke-width="2"
			stroke-dasharray="6,3"
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
			onclick={(e) => {
				e.stopPropagation(); // so it doesnt also trigger cancelAttack prevent event bubbling
				tryAttack(index, false);
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
			onclick={(e) => {
				e.stopPropagation(); // so it doesnt also trigger cancelAttack prevent event bubbling
				beginAttack(index, e);
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
			background-color: blue;
		}
		&.enemy {
			align-self: flex-start;
			transform: translateY(-30px);
			background-color: red;
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
