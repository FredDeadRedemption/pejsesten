<script lang="ts">
	import type { MinionEntity } from '$lib/shared/types';

	let { card }: { card: MinionEntity } = $props();

	const attackBuffed = $derived(card.attack > card.baseAttack);
	const attackDebuffed = $derived(card.attack < card.baseAttack);
	const defenceBuffed = $derived(card.defence > card.baseDefence);
	const defenceDebuffed = $derived(card.defence < card.baseDefence);
</script>

<div id="card" class={card.color} class:exhausted={card.exhausted}>
	<div class="art-frame">
		<img
			src={card.image_url === '' ? '/media/cards/missing-texture.jpg' : `/media/${card.image_url}`}
			alt=""
			draggable="false"
		/>
	</div>
	<div class="name">{card.name}</div>
	<div class="card-bottom">
		<div class="stat atk" class:buffed={attackBuffed} class:debuffed={attackDebuffed}>
			{card.attack}
		</div>
		<div class="stat def" class:buffed={defenceBuffed} class:debuffed={defenceDebuffed}>
			{card.defence}
		</div>
	</div>
</div>

<style lang="scss">
	#card {
		width: 100px;
		height: 147px;
		border: 2px solid $black;
		border-radius: 3px;
		overflow: hidden;
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
		&.debuffed {
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
