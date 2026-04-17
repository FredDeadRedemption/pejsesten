<script lang="ts">
	import { getIcon } from '$lib/icons';
	import type { IncantationCard } from '$lib/shared/bindings/IncantationCard';
	import type { IncantationEntity } from '$lib/shared/bindings/IncantationEntity';
	import type { MinionCard } from '$lib/shared/bindings/MinionCard';
	import type { MinionEntity } from '$lib/shared/bindings/MinionEntity';
	import { isTradeable } from '$lib/shared/lib';

	let {
		card,
		compact = false
	}: {
		card: MinionEntity | IncantationEntity | MinionCard | IncantationCard;
		compact?: boolean;
	} = $props();

	const isEntity = 'cost' in card;
	const cost = isEntity ? (card as MinionEntity).cost : (card as MinionCard).base_cost;
	const name = isEntity ? (card as MinionEntity).card.name : (card as MinionCard).name;
	const description = isEntity
		? (card as MinionEntity).card.description
		: (card as MinionCard).description;
	const color = isEntity ? (card as MinionEntity).card.color : (card as MinionCard).color;
	const imageUrl = isEntity
		? (card as MinionEntity).card.image_url
		: (card as MinionCard).image_url;
	const minion = isEntity ? 'attack' in card : 'base_attack' in card;
</script>

<div id="card" class:compact class={color}>
	<div id="content">
		<div class="card-top">
			<div class="cost-gem">{cost}</div>
			<div class="card-name">{name}</div>
			{#if isTradeable(card)}
				<div class="tradeable-badge">
					<span class="icon">
						{@html getIcon('tradeable')}
					</span>
				</div>
			{:else}
				<div style="width: 28px"></div>
			{/if}
		</div>

		<div class="art-frame" class:minion>
			<img
				src={imageUrl === '' ? '/media/cards/missing-texture.jpg' : `/media/${imageUrl}`}
				alt=""
				draggable="false"
			/>
		</div>

		<div class="description">
			<span class="icon-bg">{@html getIcon('manaWhite')}</span>
			<span class="text">{@html description}</span>
		</div>

		<div class="card-bottom">
			{#if minion}
				<div class="stat atk">
					{isEntity ? (card as MinionEntity).attack : (card as MinionCard).base_attack}
				</div>
				<div class="race">
					{isEntity
						? (card as MinionEntity).card.races.join(' · ')
						: (card as MinionCard).races.join(' · ')}
				</div>
				<div class="stat def">
					{isEntity ? (card as MinionEntity).defence : (card as MinionCard).base_defence}
				</div>
			{:else}
				<div class="spell-label">incantation</div>
			{/if}
		</div>
	</div>
</div>

<style lang="scss">
	#card {
		width: 170px;
		height: 250px;
		border: 2px solid $black;
		border-radius: 3px;
		overflow: hidden;
		user-select: none;
		-webkit-user-drag: none;
		-webkit-user-select: none;
		-moz-user-select: none;
		-ms-user-select: none;
		box-shadow: $box-shadow-primary;
		background-size: cover;
		background-repeat: no-repeat;
		background-image: url('/media/cards/card-bg-white.webp');
		outline: 1px solid $mana-white;
		padding: 3px 1px;

		&.black {
			background-image: url('/media/cards/card-bg-black.webp');
			outline: 1px solid $mana-black;
		}

		&.compact {
			scale: 0.6;
		}
	}

	#content {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.card-top {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 4px 4px 2px;
		gap: 4px;
	}

	.cost-gem {
		width: 28px;
		height: 28px;
		border-radius: 1px;
		background: rgb(87, 152, 205);
		border: 2px solid $black;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 14px;
		font-weight: 500;
		color: $white;
		flex-shrink: 0;
	}

	.card-name {
		flex: 1;
		text-align: center;
		font-size: 0.75rem;
		font-weight: 500;
		color: $black;
		line-height: 1.2;
	}

	.tradeable-badge {
		width: 28px;
		height: 28px;
		border-radius: 2px;
		background: rgba(210, 193, 171, 0.7);
		border: 2px solid $black;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 16px;
		color: $black;
		flex-shrink: 0;
		.icon {
			color: rgb(106, 93, 77);
		}
	}

	.art-frame {
		width: 97%;
		height: 38%;
		border: 2px solid $black;
		border-radius: 3px 3px 0px 0px;
		background-color: $grey-black;
		overflow: hidden;
		flex-shrink: 0;

		img {
			width: 100%;
			height: 100%;
			object-fit: cover;
		}
	}

	.keyword-row {
		display: flex;
		flex-wrap: wrap;
		gap: 3px;
		justify-content: center;
		padding: 4px 6px 2px;
	}

	.keyword {
		font-size: 0.55rem;
		background: rgba(76, 29, 149, 0.15);
		color: #4c1d95;
		border: 0.5px solid #4c1d95;
		border-radius: 10px;
		padding: 1px 6px;
		text-transform: capitalize;
	}

	.divider {
		width: 90%;
		height: 1px;
		background: rgba(0, 0, 0, 0.2);
		margin: 2px 0;
	}

	.description {
		flex: 1;
		width: 97%;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		border-left: 2px solid $black;
		border-right: 2px solid $black;
		font-size: 0.65rem;
		background: rgba(210, 193, 171, 0.7);
		overflow: hidden;

		.icon-bg {
			position: absolute;
			scale: 5;
			opacity: 0.15;
			color: #7c7c7c;
			pointer-events: none;
		}

		.text {
			padding: 5px;
			text-align: center;
			z-index: 1;
			position: relative;
		}
	}

	.card-bottom {
		border: 1px solid red;
		width: 97%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		border: 2px solid $black;
		border-top: none;
		border-bottom-left-radius: 3px;
		border-bottom-right-radius: 3px;
		padding: 3px;
		background: rgba(210, 193, 171, 0.7);
		min-height: 24px;
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
	}

	.race {
		font-size: 0.65rem;
		color: #444;
		text-align: center;
		flex: 1;
		font-style: italic;
	}

	.spell-label {
		font-size: 0.55rem;
		color: #444;
		text-align: center;
		width: 100%;
		letter-spacing: 0.06em;
		font-style: italic;
	}
</style>
