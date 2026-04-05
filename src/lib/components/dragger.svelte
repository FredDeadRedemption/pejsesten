<script lang="ts">
	let {
		x = $bindable(100),
		y = $bindable(100),
		moving = $bindable(false),
		children
	}: {
		x: number;
		y: number;
		moving: boolean;
		children: any;
	} = $props();

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
