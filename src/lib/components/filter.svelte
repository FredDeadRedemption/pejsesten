<script lang="ts">
	import { getIcon } from '$lib/icons';
	import type { Card } from '$lib/shared/types';

	let {
		cards = $bindable(),
		filteredCards = $bindable()
	}: { cards: Card[]; filteredCards: Card[] } = $props();

	let searchTerm: string = $state('');

	let showMinions: boolean = $state(false);
	let showIncantations: boolean = $state(false);

	// x is the variable that should not be reset
	const resetCardTypes = (x: number) => {
		if (x != 1) showMinions = false;
		if (x != 4) showIncantations = false;
	};

	let showGreen: boolean = $state(false);
	let showOrange: boolean = $state(false);
	let showRed: boolean = $state(false);
	let showPurple: boolean = $state(false);
	let showWhite: boolean = $state(false);
	let showBlack: boolean = $state(false);

	const filter = () => {
		filteredCards = cards?.filter((card: Card) => {
			const a = card.name.toLowerCase().includes(searchTerm.toLowerCase());
			const b = showGreen ? card.color === "black" : true;
			const c = showOrange ? card.color === "black" : true;
			const d = showRed ? card.color === "black" : true;
			const e = showPurple ? card.color === "black" : true;
			const f = showWhite ? card.color === "white" : true;
			const g = showBlack ? card.color === "black" : true;
			const h = showMinions ? card.type === "minion" : true;
			const i = showIncantations ? card.type === "incantation" : true;

			return a && b && c && d && e && f && g && h && i;
		});
	};
	filter();
</script>

<div class="bar-wrapper">
	<input
		type="text"
		name="search"
		id=""
		placeholder="Search Catalog"
		bind:value={searchTerm}
		oninput={filter}
	/>
	<div class="type-switch">
		<button
			class="button minion-trigger"
			class:active={showMinions}
			onclick={() => {
				resetCardTypes(1);
				showMinions = !showMinions;
				filter();
			}}>Minions</button
		>
		<button 
			class="button incantation-trigger"
			class:active={showIncantations}
			onclick={() => {
				resetCardTypes(4);
				showIncantations = !showIncantations;
				filter();
			}}>Incantation</button
		>
	</div>
  <!-- Might use these later if we add more colors
	<button
		class="button mana green"
		class:active={showGreen}
		onclick={() => {
			showGreen = !showGreen;
			filter();
		}}><span class="icon green">{@html getIcon('manaGreen')}</span></button
	>
	<button
		class="button mana orange"
		class:active={showOrange}
		onclick={() => {
			showOrange = !showOrange;
			filter();
		}}><span class="icon orange">{@html getIcon('manaOrange')}</span></button
	>
	<button
		class="button mana red"
		class:active={showRed}
		onclick={() => {
			showRed = !showRed;
			filter();
		}}><span class="icon red">{@html getIcon('manaRed')}</span></button
	>
	<button
		class="button mana purple"
		class:active={showPurple}
		onclick={() => {
			showPurple = !showPurple;
			filter();
		}}><span class="icon purple">{@html getIcon('manaPurple')}</span></button
	>
-->
	<button
		class="button mana white"
		class:active={showWhite}
		onclick={() => {
			showWhite = !showWhite;
			filter();
		}}><span class="icon white">{@html getIcon('manaWhite')}</span></button
	>
	<button
		class="button mana black"
		class:active={showBlack}
		onclick={() => {
			showBlack = !showBlack;
			filter();
		}}><span class="icon black">{@html getIcon('manaBlack')}</span></button
	>
</div>

<style lang="scss">
	.type-switch {
		display: flex;
	}
	.minion-trigger,
	.incantation-trigger {
		border: 1px solid $grey-mid;
		border-right: none;
		background-color: $grey-ultralight;
		color: $grey-dark;
		padding: 8px;
		&.active {
			color: $white;
			background-color: $secondary;
		}
		&:hover {
			cursor: pointer;
		}
	}
	.minion-trigger {
		border-top-left-radius: 5px;
		border-bottom-left-radius: 5px;
	}
	.incantation-trigger {
		border-right: 1px solid $grey-mid;
		border-top-right-radius: 5px;
		border-bottom-right-radius: 5px;
	}
	.mana {
		overflow: hidden;
		border: 1px solid $grey-mid;
		border-radius: 5px;
		align-self: center;
		height: 40px;
		width: 40px;
		.icon {
			scale: 2;
			color: $white;
		}
		&:hover {
			cursor: pointer;
		}
		&.green {
			background-color: $grey-ultralight;
			.icon {
				color: $green;
			}
			&.active {
				background-color: $green;
				.icon {
					color: $white;
				}
			}
		}
		&.orange {
			background-color: $grey-ultralight;
			.icon {
				color: $mana-orange;
			}
			&.active {
				background-color: $mana-orange;
				.icon {
					color: $white;
				}
			}
		}
		&.red {
			background-color: $grey-ultralight;
			.icon {
				color: $red;
			}
			&.active {
				background-color: $red;
				.icon {
					color: $white;
				}
			}
		}
		&.purple {
			background-color: $grey-ultralight;
			.icon {
				color: $mana-purple;
			}
			&.active {
				background-color: $mana-purple;
				.icon {
					color: $white;
				}
			}
		}
		&.white {
			background-color: $grey-ultralight;
			.icon {
				color: $white;
			}
			&.active {
				background-color: $white;
				.icon {
					color: $mana-black;
				}
			}
		}

		&.black {
			background-color: $grey-ultralight;
			.icon {
				color: $mana-black;
			}
			&.active {
				background-color: $mana-black;
				.icon {
					color: $white;
				}
			}
		}
	}
	input {
		border: none;
		border: 1px solid $grey-mid;
		background-color: $grey-ultralight;
		outline: none;
		color: $grey-ultradark;
		border-radius: 5px;
		padding: 10px;
	}
	.bar-wrapper {
		padding: 10px;
		background-color: $grey-light;
		border-radius: 10px;
		display: flex;
		gap: 10px;
	}
</style>
