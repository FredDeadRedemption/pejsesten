<script lang="ts">
	import Card from '$lib/components/card.svelte';
	import Filter from '$lib/components/filter.svelte';
	import { getIcon } from '$lib/icons.js';
	import { getRandomDeckName } from '$lib/util.js';
	import { slide } from 'svelte/transition';
	import type { Card as CardT } from '$lib/shared/types';
	import { getCards } from '$lib/shared/cards.js';
	import { goto } from '$app/navigation';

	let filteredCards = $state<CardT[]>([]);

	type Deck = {
		id: number;
		name: string;
		cards: number[]; // array of card id's
	};

	let cards = $state(getCards());

	// Save decks to localStorage whenever they change
	$effect(() => {
		localStorage?.setItem('decks', JSON.stringify(decks));
	});

	const getCardData = (id: number) => cards.find((card) => card.id === id);

	let decks = $state<Deck[]>(JSON.parse(localStorage?.getItem('decks') ?? '[]'));

	let selectedDeckID: number | null = $state(null);
	let deck: number[] = $state([]); // contains id's of all cards
	let deckUniques: number[] = $derived([...new Set(deck)]); // contains id's of all cards (no duplicates)
	let selectedDeckName: string | null = $state(null);
	let inspectingDeck: boolean = $state(false);

	const deleteDeck = (deckId: number) => {
		decks = decks.filter((d) => d.id !== deckId);
		inspectingDeck = false;
		selectedDeckID = null;
		selectedDeckName = null;
		deck = [];
	};

	const saveDeck = () => {
		const newDeck: Deck = {
			id: selectedDeckID ?? Date.now(),
			name: selectedDeckName ?? 'Unnamed Deck',
			cards: deck
		};
		const existing = decks.findIndex((d) => d.id === newDeck.id);
		if (existing !== -1) {
			decks[existing] = newDeck;
		} else {
			decks.push(newDeck);
		}
		selectedDeckID = null;
		selectedDeckName = null;
		inspectingDeck = false;
		deck = [];
	};

	const loadExistingDeck = (selectedDeckId: number) => {
		const selectedDeck = decks.find((d) => d.id === selectedDeckId);
		if (selectedDeck) {
			selectedDeckID = selectedDeck.id;
			selectedDeckName = selectedDeck.name;
			inspectingDeck = true;
			if (Array.isArray(selectedDeck.cards)) {
				deck = selectedDeck.cards.map((cardId: number) => Number(cardId)); // Convert each item to a number
			} else {
				deck = []; // Fallback to an empty array if `cards` is not an array
			}
		} else {
			selectedDeckID = null;
			deck = []; // Reset the deck if "New Deck" is selected
		}
	};

	const loadNewDeck = () => {
		inspectingDeck = true;
		selectedDeckName = getRandomDeckName();
		deck = [];
	};

	const exportDecks = () => {
		const data = localStorage?.getItem('decks') ?? '[]';
		const base64 = btoa(data);
		navigator.clipboard.writeText(base64);
		alert('Decks copied to clipboard!');
	};

	const importDecks = () => {
		const input = prompt('Paste your deck data here:');
		if (!input) return;
		try {
			const decoded = atob(input);
			const parsed = JSON.parse(decoded);
			if (!Array.isArray(parsed)) throw new Error();
			decks = parsed;
			localStorage?.setItem('decks', decoded);
		} catch {
			alert('Invalid deck data!');
		} 
	};
</script>

<div class="main">
	<div class="catalog-search-wrapper">
		<Filter bind:cards bind:filteredCards></Filter>
		<div class="card-wrapper">
			{#each filteredCards as card (card.id)}
				<!-- svelte-ignore a11y_consider_explicit_label -->
				<button
					class="invisible"
					onclick={() => {
						if (deck.length >= 50) return; // deck cant have more than 50 cards
						deck.push(card.id);
					}}
				>
					<Card {card}></Card>
				</button>
			{/each}
		</div>
	</div>
	<div class="deck">
		<!-- RENDER CARDS IN SELECTED DECK -->
		{#if inspectingDeck}
			<div class="name-length">
				<input id="name-input" type="text" maxlength="36" bind:value={selectedDeckName} />
				<span class="length">{deck.length} / 50</span>
			</div>
			<div class="cards">
				{#each deckUniques as id}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<div
						transition:slide={{ axis: 'x', duration: 250 }}
						class="card-in-deck-view"
						style="background-image: url({getCardData(id)?.image_url});"
						onclick={() => {
							const cardIndex = deck.findIndex((c) => c === id);
							if (cardIndex !== -1) deck.splice(cardIndex, 1);
						}}
					>
						<span class="name">{getCardData(id)?.name}</span><span class="count"
							>x {deck.filter((c) => c === id).length}</span
						>
					</div>
				{/each}
			</div>
			<div class="back-delete-btn-grp">
				<button class="button primary back" onclick={saveDeck}> Done </button>
				<button class="delete" onclick={() => deleteDeck(selectedDeckID!)}>
					<span class="icon">{@html getIcon('delete')}</span>
				</button>
			</div>
		{:else}
			<!-- RENDER ALL USERS DECKS -->
			{#if decks.length > 0}
				{#each decks as deck}
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<div
						transition:slide={{ axis: 'x', duration: 250 }}
						class="deck-in-deck-view"
						style="background-image: url(TODO MAKE SOME KINDA IMAGE HERE);"
						onclick={() => {
							loadExistingDeck(deck.id);
						}}
					>
						<span class="name">{deck?.name}</span>
					</div>
				{/each}
			{:else}
				<p>Seems you have no decks</p>
			{/if}

			<button
				class="button primary new"
				onclick={() => {
					loadNewDeck();
				}}>New Deck</button
			>
			<button
				class="button primary import"
				onclick={() => {
					importDecks();
				}}>Import Decks</button
			>
			<button
				class="button primary export"
				onclick={() => {
					exportDecks();
				}}>Export Decks</button
			>
			<button class="button primary play"
			onclick={() => goto("/")}>Play</button>
		{/if}
	</div>
</div>

<style lang="scss">
	button.play {
		width: 100%;
		margin-top: 10px;

	}
	button.import,
	button.export {
		width: 100%;
		margin-top: 10px;
	}
	.name-length {
		display: grid;
		grid-template-columns: 1fr 40px;
		.length {
			display: flex;
			font-size: 0.8rem;
			align-items: center;
			justify-content: center;
			background-color: $secondary;
			color: $white;
		}
	}
	.catalog-search-wrapper {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}
	.deck {
		background-color: $grey-light;
		border-radius: 3px;
		overflow: scroll;
		display: flex;
		height: 500px;
		flex-direction: column;
		.cards {
			overflow: scroll;
		}
		.card-in-deck-view {
			display: flex;
			justify-content: space-between;
			align-items: center;
			height: 40px;
			width: 100%;
			background-size: cover;
			color: $white;
			user-select: none;
			font-weight: 800;
			&:hover {
				cursor: pointer;
			}
			.name {
				display: flex;
				align-items: center;
				white-space: nowrap;
				flex-grow: 1;
				padding-left: 10px;
				height: 100%;
				background: linear-gradient(to right, rgb(0, 0, 0) 0%, rgba(31, 31, 31, 0) 100%);
			}
			.count {
				display: flex;
				align-items: center;
				justify-content: flex-end;
				white-space: nowrap;
				width: 50px;
				padding-right: 10px;
				height: 100%;
				background: linear-gradient(to left, rgb(0, 0, 0) 0%, rgba(31, 31, 31, 0) 100%);
			}
		}
		.deck-in-deck-view {
			display: flex;
			justify-content: space-between;
			align-items: center;
			height: 40px;
			width: 100%;
			background-size: cover;
			color: $white;
			user-select: none;
			font-weight: 800;
			&:hover {
				cursor: pointer;
			}
			.name {
				display: flex;
				align-items: center;
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;
				flex-grow: 1;
				padding-left: 10px;
				height: 100%;
				background: linear-gradient(to right, rgb(0, 0, 0) 0%, rgba(31, 31, 31, 0) 100%);
			}
		}
	}
	.invisible {
		box-sizing: auto !important;
		border: none;
		background-color: transparent;
		border-radius: 5px;
		&:hover {
			outline: 2px solid $secondary;
			z-index: 3;
			cursor: pointer;
		}
	}
	#name-input {
		height: 40px;
		font-weight: bold;
		background-color: $primary;
		border: none;
		color: $white;
		text-align: center;
		padding: 5px;
		&:focus {
			outline: none;
		}
	}
	.new {
		margin-top: auto;
		min-width: 100%;
	}
	.back {
		min-width: 100%;
		border-top-left-radius: 0px;
		border-bottom-right-radius: 0px;
		border-top-right-radius: 0px;
	}
	.delete {
		background-color: $secondary;
		border: none;
		color: $white;
		&:hover {
			background-color: $black;
			cursor: pointer;
		}
	}
	.back-delete-btn-grp {
		margin-top: auto;
		display: grid;
		grid-template-columns: 1fr 35px;
	}
	.main {
		margin: 30px;
		display: grid;
		grid-template-columns: 1fr 250px;
		gap: 10px;
	}
	.card-wrapper {
		padding: 10px;
		background-color: $grey-light;
		border-radius: 10px;
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
	}
</style>
