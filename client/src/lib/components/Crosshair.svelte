<script lang="ts">
	import { mouse } from '$lib/mouse.svelte';
	import { targeting } from '$lib/targeting.svelte';
</script>

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

<style lang="scss">
	.crosshair {
		position: fixed;
		transform: translate(-50%, -50%);
		pointer-events: none;
		z-index: 9999;
		cursor: none;
		filter: drop-shadow(0 0 4px #645d5b);
	}
</style>
