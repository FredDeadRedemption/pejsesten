<script lang="ts">
	import CardSmall from './cardSmall.svelte';
	import { attack } from '$lib/socket/socket';
	import type { MinionCard } from '$lib/shared/types';

	let {
		selfBattleField = $bindable(),
		enemyBattleField = $bindable(),
		selfHP = $bindable(),
		enemyHP = $bindable()
	}: {
    selfBattleField: MinionCard[];
    enemyBattleField: MinionCard[];
    selfHP: number;
    enemyHP: number;
  } = $props();

	let origin: number | null = $state(null);

	const beginAttack = (index: number) => {
		console.log('ATTACKING WITH INDEX: ', index);
		origin = index;
	};

	const tryAttack = (index: number, face: boolean) => {
		if (origin === null) return;
		console.log('TRYING TO ATTACK INDEX: ', index, 'FACE: ', face);
		attack({
			origin: origin,
			target: index,
			face: face
		});
		origin = null;
	};

	const cancelAttack = () => {
		console.log('CANCEL ATTACK');
		origin = null;
	};
</script>

<svelte:window onclick={cancelAttack} />

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
			<CardSmall card={card!}></CardSmall>
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
				beginAttack(index);
			}}
		>
			<CardSmall card={card!}></CardSmall>
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
</style>
