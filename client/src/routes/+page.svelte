<script lang="ts">
	import {
		queueUp,
		leaveQueue,
		invalidateSocket,
		connectSocket,
		resetServer,
		queueUpBot
	} from '$lib/socket.svelte';
	import { fly } from 'svelte/transition';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import type { PlayerMetaData } from '$lib/shared/bindings/PlayerMetaData';

	type Deck = {
		id: number;
		name: string;
		cards: number[]; // array of card id's
	};

	let decks: Deck[] = $state([]);
	let choosenDeckJson = $state<number[]>([]);

	onMount(() => {
		decks = JSON.parse(localStorage?.getItem('decks') ?? '[]');
		choosenDeckJson = decks[0]?.cards ?? [];
	});

	const DEV_URL = 'http://localhost:3000/';
	const PRODUCTION_URL = 'https://pejsesten.finrod.dk';

	let playerMetaData: PlayerMetaData = $derived({
		username: 'Out-of-Towner',
		choosen_deck: choosenDeckJson,
		avatar: 'uaogidsogijsogij'
	});

	let ellipseVar = $state('.');
	setInterval(() => (ellipseVar = ellipseVar.length >= 3 ? '.' : ellipseVar + '.'), 300);

	let queuedUp = $state(false);

	const handleReset = (prod: boolean) => {
		invalidateSocket();
		connectSocket(prod ? PRODUCTION_URL : DEV_URL);
		resetServer();
	};
</script>

<main class="main">
	<p>Playin as <strong>{'random troldmayn'}</strong></p>

	<div style="display: flex; flex-direction: row; gap: 10px;">
		<select bind:value={choosenDeckJson}>
			{#each decks as deck}
				<option value={deck.cards}>{deck.name}</option>
			{/each}
		</select>
		
		<button class="button primary" onclick={() => goto("/deck")}>Make Deck</button>
		<button class="button primary" onclick={() => goto("/blog/gamedesign")}>Game Design</button>
	</div>

	<div style="display: flex; flex-direction: row; gap: 10px;">
		<div class="right" style="display: flex; flex-direction: column; gap: 10px;">
			<p class="dev">Development</p>
			<button class="button primary" onclick={() => handleReset(false)}>Reset Server</button>

			<button
				class="button primary"
				class:queuedUp
				onclick={() => {
					connectSocket(DEV_URL);
					if (queuedUp) {
						leaveQueue();
						invalidateSocket();
					} else queueUp(playerMetaData);
					queuedUp = true;
				}}
			>
				{queuedUp ? `Queueing ${ellipseVar}` : 'Join Queue'}
			</button>
      <button
				class="button primary"
				class:queuedUp
				onclick={() => {
					connectSocket(DEV_URL);
					if (queuedUp) {
						leaveQueue();
						invalidateSocket();
					} else queueUpBot(playerMetaData);
					queuedUp = true;
				}}
			>
				{queuedUp ? `Queueing ${ellipseVar}` : 'Play Bot'}
			</button>
		</div>
		<div class="left" style="display: flex; flex-direction: column; gap: 10px;">
			<p class="prod">Production</p>

			<button class="button primary" onclick={() => handleReset(true)}>Reset Server</button>
			<button
				class="button primary"
				class:queuedUp
				onclick={() => {
					connectSocket(PRODUCTION_URL);
					if (queuedUp) {
						leaveQueue();
						invalidateSocket();
					} else queueUp(playerMetaData);
					queuedUp = true;
				}}
			>
				{queuedUp ? `Queueing ${ellipseVar}` : 'Join Queue'}
			</button>
      <button
				class="button primary"
				class:queuedUp
				onclick={() => {
					connectSocket(PRODUCTION_URL);
					if (queuedUp) {
						leaveQueue();
						invalidateSocket();
					} else queueUpBot(playerMetaData);
					queuedUp = true;
				}}
			>
				{queuedUp ? `Queueing ${ellipseVar}` : 'Play Bot'}
			</button>
		</div>
	</div>

	{#if queuedUp}
		<!-- svelte-ignore a11y_consider_explicit_label -->
		<button
			class="button primary"
			transition:fly={{ duration: 250 }}
			onclick={() => {
				leaveQueue();
				queuedUp = false;
			}}
		>
			Leave Queue
		</button>
	{/if}
</main>

<style lang="scss">
	select {
		padding: 10px;
		background-color: $grey-light;
		color: steelblue;
		&:hover {
			cursor: pointer;
		}
	}
	.left {
		border: 1px dotted steelblue;
		padding: 10px;
	}
	.right {
		border: 1px dotted rgb(247, 125, 38);
		padding: 10px;
	}
	.dev {
		color: rgb(247, 125, 38);
	}
	.prod {
		color: steelblue;
	}
	p {
		color: $grey-ultralight;
	}
	.queuedUp {
		background-color: $grey-mid;
		&:hover {
			background-color: $grey-mid;
			cursor: auto;
		}
	}
	.main {
		margin: 30px;
		display: flex;
		flex-direction: column;
		width: 100%;
		align-items: center;
		text-align: center;
		gap: 10px;
	}
</style>
