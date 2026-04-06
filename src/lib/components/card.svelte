<script lang="ts">
	import { getIcon } from '$lib/icons';
	import { colorMap } from '$lib/util';
	import type { Card, CardEntity } from '$lib/shared/types';

	// Card is for ui representation, CardEntity is for rendering game state.
	let { card, compact = false }: { card: CardEntity | Card; compact?: boolean } = $props();

	let theme = $derived(colorMap[card.color]);
</script>

<div id="card" class={theme.bg} class:compact>
	<div id="content">
		<div class="img-wrap">
			<img
				src={card.image_url === ''
					? '/media/cards/missing-texture.jpg'
					: `/media/${card.image_url}`}
				alt=""
				draggable="false"
			/>
		</div>
		<div class="tilte {theme.title}">
			<span>{card.name}</span>
		</div>
		<div class="description {theme.desc}">
			<div class="description-inner">
				<span class="icon-big" style="color: {theme.color}">{@html getIcon(theme.icon)}</span>
				<span class="text">{card.description}</span>
			</div>
			{#if card.type === 'minion'}
				<div class="bottom {theme.title}">
					{card.baseAttack} | {card.baseDefence}
				</div>
			{/if}
		</div>
	</div>
</div>

<style lang="scss">
	.description-inner {
		display: grid;
		//place-items: center;
		flex: 1;
		width: 100%;

		.icon-big {
			align-self: center;
			justify-self: center;
			transform: translateY(5px);
			grid-area: 1 / 1;
		}

		.icon-big {
			scale: 6;
			opacity: 0.5;
		}

		.text {
			padding: 5px;
			z-index: 1;
		}
	}
	.icon {
		font-size: 0.6rem;
	}
	.red-bg {
		outline: 1px solid $red;
		background-image: url('/media/cards/card-bg-red.webp');
	}
	.green-bg {
		outline: 1px solid $green;
		background-image: url('/media/cards/card-bg-green.webp');
	}
	.white-bg {
		outline: 1px solid $mana-white;
		background-image: url('/media/cards/card-bg-white.webp');
	}
	.orange-bg {
		outline: 1px solid $mana-orange;
		background-image: url('/media/cards/card-bg-brown.webp');
	}
	.purple-bg {
		outline: 1px solid $mana-purple;
		background-image: url('/media/cards/card-bg-purple.webp');
	}
	.black-bg {
		outline: 1px solid $mana-black;
		background-image: url('/media/cards/card-bg-black.webp');
	}
	#card {
		//scale: 0.6; // game scale
		border: 2px solid $black;
		// width: 170px;
		// height: 250px;
		width: 170px;
		height: 250px;
		box-shadow: $box-shadow-primary;
		border-radius: 3px;
		overflow: hidden;
		user-select: none; // Prevents selection
		-webkit-user-drag: none;
		-webkit-user-select: none;
		-moz-user-select: none;
		-ms-user-select: none;
		padding: 3px 1px;
		background-size: cover;
		background-repeat: no-repeat;
		&.compact {
			scale: 0.6;
		}
		#content {
			// overflow: hidden;
			background-size: cover;
			background-repeat: no-repeat;
			width: 100%;
			height: 100%;
			display: flex;
			flex-direction: column;
			align-items: center;

			.img-wrap {
				border: 2px solid $black;
				border-bottom: none;
				border-radius: 2px;
				width: 96%;
				height: 40%;
				background-color: $grey-black;
				color: $white;
				display: flex;
				justify-content: center;
				align-items: center;
				overflow: hidden;
				img {
					z-index: 1;
					height: 100%;
					width: 100%;
					object-fit: cover;
				}
			}
			.tilte {
				width: 100%;
				border: 2px solid $black;
				border-radius: 3px;
				display: flex;
				align-items: center;
				gap: 2px;
				width: 100%;
				height: 22px;
				padding: 2px;
				.cost {
					color: $white;
					display: flex;
					justify-content: center;
					align-items: center;
					width: 14px;
					height: 14px;
					border-radius: 100px;
				}
			}
			.description {
				display: flex;
				flex-direction: column;
				justify-content: space-between;
				align-items: center;
				border-bottom-right-radius: 3px;
				border-bottom-left-radius: 3px;
				width: 97%;
				border-left: 2px solid $black;
				border-right: 2px solid $black;
				border-bottom: 2px solid $black;
				font-size: 0.7rem;
				flex-grow: 1;
				.text {
					padding: 5px;
					text-align: center;
				}
			}
			.bottom {
				font-weight: 800;
				transform: translateY(8px);
				width: 40%;
				justify-self: flex-end;
				text-align: center;
				background-color: rgb(200, 186, 186);
				border: 2px solid $black;
				border-radius: 3px;
				padding: 3px;
				justify-content: space-between;
			}
		}
	}
	.white {
		background: linear-gradient(to right, rgb(201, 201, 201) 0%, rgba(200, 185, 169, 0.8) 100%);
	}
	.white-desc {
		background: rgba(210, 193, 171, 0.7);
	}

	.black {
		background: linear-gradient(to right, rgb(162, 162, 162) 0%, rgba(93, 93, 93, 0.8) 100%);
	}
	.black-desc {
		background: rgba(179, 179, 179, 0.7);
	}

	.purple {
		background: linear-gradient(to right, rgb(188, 203, 254) 0%, rgba(150, 155, 190, 0.8) 100%);
	}
	.purple-desc {
		background: rgba(175, 175, 190, 0.7);
	}

	.green {
		background: linear-gradient(to right, rgb(189, 216, 176) 0%, rgba(160, 180, 150, 0.8) 100%);
	}
	.green-desc {
		background: rgba(170, 189, 159, 0.8);
	}

	.red {
		background: linear-gradient(to right, rgb(240, 166, 166) 0%, rgba(182, 55, 55, 0.8) 100%);
	}
	.red-desc {
		background: rgba(255, 178, 178, 0.8);
	}
	.orange {
		background: linear-gradient(to right, rgb(180, 147, 119) 0%, rgba(163, 99, 60, 0.7) 100%);
	}
	.orange-desc {
		background: rgba(212, 186, 163, 0.9);
	}
</style>
