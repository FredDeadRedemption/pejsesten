<script lang="ts">
	import { fly } from 'svelte/transition';
	import Card from './Card.svelte';
	import { portal } from '$lib/attachments';
	import type { MinionEntity } from '$lib/shared/bindings/MinionEntity';

	let { card }: { card: MinionEntity } = $props();

	const attackBuffed = $derived(card.attack > card.card.base_attack);
	const attackDebuffed = $derived(card.attack < card.card.base_attack);
	const isBuffed = $derived(card.max_defence > card.card.base_defence);
	const isDamaged = $derived(card.defence < card.max_defence);

	let hoverTimer: ReturnType<typeof setTimeout> | null = null;
	let showPreview = $state(false);

	let previewX = $state(0);
	let previewY = $state(0);

	const onMouseEnter = (e: MouseEvent) => {
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    previewX = rect.right + 8;
    previewY = rect.top + rect.height / 2;
    hoverTimer = setTimeout(() => { showPreview = true; }, 200);
};

	const onMouseLeave = () => {
		if (hoverTimer) {
			clearTimeout(hoverTimer);
			hoverTimer = null;
		}
		showPreview = false;
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	id="card"
	class={card.card.color}
	class:exhausted={card.exhausted}
	class:guard={card.card.attributes.includes('Guard')}
	onmouseenter={onMouseEnter}
	onmouseleave={onMouseLeave}
>
	{#if card.ward_active}
		<div class="ward-overlay"></div>
	{/if}
	{#if card.stealth_active}
		<div class="stealth-overlay"></div>
	{/if}
	{#if showPreview}
		<div {@attach portal} transition:fly={{ duration: 150, y: 15 }} class="preview"
		style="left: {previewX}px; top: {previewY}px;">

			<Card {card} />
		</div>
	{/if}
	<div class="art-frame">
		<img
			src={card.card.image_url === '' ? '/media/cards/missing-texture.jpg' : `/media/${card.card.image_url}`}
			alt=""
			draggable="false"
		/>
	</div>
	<div class="name">{card.card.name}</div>
	<div class="card-bottom">
		<div class="stat atk" class:buffed={attackBuffed} class:debuffed={attackDebuffed}>
			{card.attack}
		</div>
		<div class="stat def" class:buffed={isBuffed && !isDamaged} class:damaged={isDamaged}>
			{card.defence}
		</div>
	</div>
</div>

<style lang="scss">
	@use "../../vars" as *;
	#card {
		position: relative;
		width: 100px;
		height: 147px;
		border: 2px solid $black;
		border-radius: 3px;
		overflow: visible;
		user-select: none;
		-webkit-user-select: none;
		-moz-user-select: none;
		-ms-user-select: none;
		box-shadow: $box-shadow-primary;
		background-size: cover;
		background-repeat: no-repeat;
		background-image: url('/media/cards/card-bg-white.webp');
		outline: 1px solid $mana-white;
		padding: 2px 1px;
		display: flex;
		flex-direction: column;

		&.black {
			background-image: url('/media/cards/card-bg-black.webp');
			outline: 1px solid $mana-black;
		}

		&.exhausted {
			outline: 2px solid $red;
		}

		&.guard {
			//outline: 8px solid grey;
			animation: guard-pulse 1.5s ease-in-out infinite;
		}

		@keyframes guard-pulse {
					0%, 100% { box-shadow: 0 0 0 10px #888, 0 0 2px 1px rgba(160, 160, 160,
			0.9); }
					50%       { box-shadow: 0 0 0 8px #bbb, 0 0 3px 1px rgba(200, 200, 200,
			0.8); }
			}
	}

	.ward-overlay {
		position: absolute;
		inset: 0;
		border-radius: 3px;
		pointer-events: none;
		background: linear-gradient(135deg, rgba(255, 215, 0, 0.18) 0%, rgba(255, 255, 180, 0.08) 100%);
		box-shadow: inset 0 0 8px 2px rgba(255, 215, 0, 0.45);
		z-index: 2;
		animation: ward-pulse 3s ease-in-out infinite;

		&::after {
			content: '';
			position: absolute;
			inset: 0;
			border-radius: 3px;
			background: linear-gradient(115deg, transparent 40%, rgba(255, 240, 120, 0.35) 50%, transparent 60%);
			background-size: 200% 200%;
			animation: ward-shimmer 7s linear infinite;
		}
	}

	@keyframes ward-pulse {
		0%, 100% { box-shadow: inset 0 0 8px 2px rgba(255, 215, 0, 0.45); }
		50%       { box-shadow: inset 0 0 14px 4px rgba(255, 215, 0, 0.75); }
	}

	@keyframes ward-shimmer {
		0%   { background-position: 200% 0%; }
		100% { background-position: -200% 0%; }
	}

	.stealth-overlay {
		position: absolute;
		inset: 0;
		border-radius: 3px;
		pointer-events: none;
		background: linear-gradient(160deg, rgba(20, 20, 35, 0.75) 0%, rgba(80, 80, 110, 0.25) 50%, rgba(10, 10, 20, 0.8) 100%);
		box-shadow: inset 0 0 14px 5px rgba(20, 20, 50, 0.85);
		z-index: 2;
		animation: stealth-drift 4s ease-in-out infinite;

		&::before {
			content: '';
			position: absolute;
			inset: 0;
			border-radius: 3px;
			background:
				radial-gradient(ellipse 60% 30% at 30% 60%, rgba(160, 160, 200, 0.35) 0%, transparent 70%),
				radial-gradient(ellipse 40% 25% at 70% 30%, rgba(130, 130, 170, 0.3) 0%, transparent 70%);
			animation: stealth-smoke 5.5s ease-in-out infinite alternate;
		}

		&::after {
			content: '';
			position: absolute;
			inset: 0;
			border-radius: 3px;
			background: linear-gradient(115deg, transparent 35%, rgba(150, 150, 170, 0.12) 50%, transparent 65%);
			background-size: 200% 200%;
			animation: stealth-shimmer 7s linear infinite;
		}
	}

	@keyframes stealth-drift {
		0%, 100% { box-shadow: inset 0 0 14px 5px rgba(20, 20, 50, 0.85); opacity: 0.8; }
		50%       { box-shadow: inset 0 0 22px 8px rgba(20, 20, 50, 1); opacity: 1; }
	}

	@keyframes stealth-smoke {
		0%   { background-position: 0% 0%, 100% 100%; opacity: 0.6; }
		100% { background-position: 100% 50%, 0% 50%; opacity: 1; }
	}

	@keyframes stealth-shimmer {
		0%   { background-position: 200% 0%; }
		100% { background-position: -200% 0%; }
	}

	.preview {
    position: fixed;
    transform: translateY(-50%);
    z-index: 1000;
    pointer-events: none;
    filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.6));
}

	.art-frame {
		position: relative;
		width: 97%;
		height: 80px;
		border: 2px solid $black;
		border-bottom: none;
		border-radius: 3px 3px 0 0;
		background-color: $grey-black;
		overflow: hidden;
		align-self: center;

		img {
			width: 100%;
			height: 100%;
			object-fit: cover;
		}
	}

	.name {
		font-size: 0.6rem;
		font-weight: 500;
		text-align: center;
		padding: 2px;
		width: 100%;
		background: rgba(210, 193, 171, 0.85);
		border: 2px solid $black;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.card-bottom {
		width: 97%;
		align-self: center;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 3px 0px;
		min-height: 28px;
	}

	.stat {
		width: 28px;
		height: 28px;
		border-radius: 1px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 13px;
		font-weight: 800;
		border: 2px solid $black;
		flex-shrink: 0;

		&.atk {
			background: rgb(230, 179, 50);
			color: $white;
		}
		&.def {
			background: indianred;
			color: $white;
		}

		&.buffed {
			color: $green;
		}
		&.damaged {
			background-color: #d59393;
			color: $red;
		}
	}

	.race {
		font-size: 0.5rem;
		color: #444;
		text-align: center;
		flex: 1;
		font-style: italic;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		padding: 0 2px;
	}
</style>
