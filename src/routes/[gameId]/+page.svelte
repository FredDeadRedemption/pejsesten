<script lang="ts">
	import { endTurn } from '$lib/socket/socket.svelte';
	import { page } from '$app/state';
	import { gameState } from '$lib/socket/socket.svelte';
	import Hand from '$lib/components/hand.svelte';
	import Battlefields from '$lib/components/battlefields.svelte';
	import Deck from '$lib/components/deck.svelte';
	import Graveyard from '$lib/components/graveyard.svelte';
	import { enterFullscreen } from '$lib/util.js';
	import { mouse, updateMouse } from '$lib/mouse.svelte';
	import { endTargeting, targeting } from '$lib/targeting.svelte';
</script>

<svelte:window
	onmousemove={(e) => updateMouse(e.clientX, e.clientY)}
	onmouseup={() => {
		if (targeting.active) endTargeting();
	}}
/>

<div id="game-frame">
	{#if targeting.active}
		<svg
			class="crosshair"
			style="left: {mouse.x}px; top: {mouse.y}px;"
			width="100"
			height="100"
			viewBox="0 0 40 40"
			xmlns="http://www.w3.org/2000/svg"
		>
			<circle cx="20" cy="20" r="8" fill="none" stroke="#2E2E2E" stroke-width="1.5" />
			<circle cx="20" cy="20" r="1.5" fill="#2E2E2E" />
			<!-- top -->
			<line x1="20" y1="2" x2="20" y2="10" stroke="#2E2E2E" stroke-width="1.5" />
			<!-- bottom -->
			<line x1="20" y1="30" x2="20" y2="38" stroke="#2E2E2E" stroke-width="1.5" />
			<!-- left -->
			<line x1="2" y1="20" x2="10" y2="20" stroke="#2E2E2E" stroke-width="1.5" />
			<!-- right -->
			<line x1="30" y1="20" x2="38" y2="20" stroke="#2E2E2E" stroke-width="1.5" />
		</svg>
	{/if}
	<!-- DEBUGGING / LOGGIN -->
	<button id="fs"
			onclick={() => {
				enterFullscreen('game-frame');
			}}>Go Fullscreen</button
		>
	<!-- GRAVEYARDS -->
	<div class="graveyard-zone">
		<div class="enemy-graveyard">
			<Graveyard bind:graveyard={gameState.enemy.graveyard}></Graveyard>
		</div>
		<div class="self-graveyard">
			<Graveyard bind:graveyard={gameState.self.graveyard}></Graveyard>
		</div>
	</div>
	<!-- HANDS & BATTLEFIELD -->
	<div class="hand-battlefield-zone">
		<div class="enemy-hand">
			<Hand bind:hand={gameState.enemy.hand} bind:mana={gameState.enemy.mana} enemy={true}></Hand>
		</div>
		<div class="battlefield">
			<Battlefields
				bind:selfBattleField={gameState.self.battlefield}
				bind:enemyBattleField={gameState.enemy.battlefield}
				bind:selfHero={gameState.self.hero}
				bind:enemyHero={gameState.enemy.hero}
			></Battlefields>
		</div>
		<div class="self-hand">
			<Hand self={true} yourTurn={gameState.yourTurn} bind:hand={gameState.self.hand} bind:mana={gameState.self.mana}></Hand>
		</div>
	</div>
	<!-- DECKS -->
	<div class="deck-zone">
		<div class="enemy-deck">
			<Deck bind:deck={gameState.enemy.deck}></Deck>
		</div>
		<div class="self-deck">
			<Deck bind:deck={gameState.self.deck}></Deck>
		</div>
	</div>
</div>

<style lang="scss">
	.crosshair {
		position: fixed;
		transform: translate(-50%, -50%);
		pointer-events: none;
		z-index: 9999;
		cursor: none;
		filter: drop-shadow(0 0 4px #645d5b);
	}

	#fs{
		position: absolute;
		margin: 0 auto;
		right: 50vw;
	}
	#game-frame {
		overflow: hidden;
		min-width: 100%;
		height: 100vh;
		display: grid;
		grid-template-columns: 15% 1fr 15%;
	}
	.graveyard-zone,
	.deck-zone {
		display: grid;
		grid-template-rows: 1fr 1fr;
	}
	.self-deck,
	.self-graveyard {
		background-color: rgb(88, 64, 64);
	}
	.enemy-deck,
	.enemy-graveyard {
		background-color: rgb(79, 66, 87);
	}
	.hand-battlefield-zone {
		display: grid;
		grid-template-rows: 18% 1fr 18%;
	}
	.battlefield {
		background-color: rgb(70, 65, 74);
		display: grid;
		grid-template-rows: 1fr 1fr;
	}
	.self-hand,
	.enemy-hand {
		background-color: rgb(77, 71, 83);
	}
	.enemy-hand,
	.enemy-deck {
		pointer-events: none;
	}
</style>
