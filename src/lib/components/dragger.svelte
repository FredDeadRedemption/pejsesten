<script lang="ts">
	import { onMount } from "svelte";

	let { x = $bindable(100), y = $bindable(100), moving = $bindable(false), children } = $props()
	onMount(()=>{
		console.log("in BUSINESS")
	})

</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<section style="left: {x}px; top: {y}px;" class="draggable">
	{@render children()}
</section>

<style lang="scss">
	.draggable {
		user-select: none;
		cursor: move;
		position: absolute;
	}
</style>
