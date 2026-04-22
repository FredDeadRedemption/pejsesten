<script lang="ts">
	import { gameState, submitMulligan } from '$lib/socket.svelte';
	import { getEntity } from '$lib/lib';
	import Card from './Card.svelte';

	let selected = $state(new Set<number>());
	let submitted = $state(false);

	const toggle = (index: number) => {
		if (submitted) return;
		const next = new Set(selected);
		if (next.has(index)) next.delete(index);
		else next.add(index);
		selected = next;
	};

	const confirm = () => {
		submitted = true;
		submitMulligan([...selected]);
	};
</script>

<div class="mulligan-overlay">
	{#if submitted}
		<div class="waiting">
			<p>Waiting for opponent...</p>
		</div>
	{:else}
		<div class="title">Choose cards to replace</div>
		<div class="cards">
			{#each gameState.self_board.hand as raw, i}
				{@const card = getEntity(raw)}
				<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div class="card-wrap" class:selected={selected.has(i)} onclick={() => toggle(i)}>
					<Card {card} />
					{#if selected.has(i)}
						<div class="cross">✕</div>
					{/if}
				</div>
			{/each}
		</div>
		<button class="confirm-btn" onclick={confirm}>Confirm</button>
	{/if}
</div>

<style lang="scss">
	@use '../../vars' as *;

	.mulligan-overlay {
		position: fixed;
		inset: 0;
		z-index: 100;
		background: rgba(10, 8, 12, 0.88);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 32px;
	}

	.title {
		font-size: 1.2rem;
		color: $grey-ultralight;
		letter-spacing: 0.08em;
	}

	.cards {
		display: flex;
		gap: 24px;
		align-items: center;
	}

	.card-wrap {
		position: relative;
		cursor: pointer;
		transition: transform 120ms ease;

		&:hover {
			transform: translateY(-8px);
		}

		&.selected {
			filter: brightness(0.45);
		}
	}

	.cross {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 4rem;
		color: $secondary;
		pointer-events: none;
	}

	.confirm-btn {
		padding: 10px 40px;
		font-size: 1rem;
		letter-spacing: 0.1em;
		background: $primary;
		color: $white;
		border: 2px solid $grey-ultralight;
		border-radius: 3px;
		cursor: pointer;
		transition: background 120ms;

		&:hover {
			background: lighten($primary, 10%);
		}
	}

	.waiting {
		color: $grey-ultralight;
		font-size: 1.1rem;
		letter-spacing: 0.08em;
	}
</style>
