<script lang="ts">
	import Hand from '$lib/components/Hand.svelte';
	import Battlefield from '$lib/components/Battlefield.svelte';
	import Deck from '$lib/components/Deck.svelte';
	import Graveyard from '$lib/components/Graveyard.svelte';
	import { enterFullscreen } from '$lib/lib.js';
	import { updateMouse } from '$lib/mouse.svelte';
	import { endTargeting, targeting } from '$lib/targeting.svelte';
	import Crosshair from '$lib/components/Crosshair.svelte';
	import HandEnemy from '$lib/components/HandEnemy.svelte';
	import GraveyardEnemy from '$lib/components/GraveyardEnemy.svelte';
	import DeckEnemy from '$lib/components/DeckEnemy.svelte';
	import { gameState } from '$lib/socket.svelte';

	$inspect(gameState);
</script>

<svelte:window
	onmousemove={(e) => updateMouse(e.clientX, e.clientY)}
	onmouseup={() => {
		if (targeting.active) endTargeting();
	}}
/>

<div id="game-frame">
	<Crosshair />
	<!-- DEBUGGING / LOGGIN -->
	<button
		id="fs"
		onclick={() => {
			enterFullscreen('game-frame');
		}}>Go Fullscreen</button
	>
	<!-- GRAVEYARDS -->
	<div class="graveyard-zone">
		<div class="enemy-graveyard">
			<GraveyardEnemy />
		</div>
		<div class="self-graveyard">
			<Graveyard />
		</div>
	</div>
	<!-- HANDS & BATTLEFIELD -->
	<div class="hand-battlefield-zone">
		<div class="enemy-hand">
			<HandEnemy />
		</div>
		<div class="battlefield">
			<Battlefield />
		</div>
		<div class="self-hand">
			<Hand />
		</div>
	</div>
	<!-- DECKS -->
	<div class="deck-zone">
		<div class="enemy-deck">
			<DeckEnemy />
		</div>
		<div class="self-deck">
			<Deck />
		</div>
	</div>
</div>

<style lang="scss">
	#fs {
		position: absolute;
		margin: 0 auto;
		right: 10vw;
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
		grid-template-rows: 1fr 360px 1fr;
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
