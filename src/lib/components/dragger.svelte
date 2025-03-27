<script lang="ts">
	let { left = 100, top = 100, moving = $bindable(false), children } = $props()

	const onMouseDown = () => moving = true;
	const onMouseUp = () => moving = false;

	const onMouseMove = (e: { movementX: number; movementY: number; }) => {
		if (!moving) return;
		left += e.movementX;
		top += e.movementY;
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<section onmousedown={onMouseDown} style="left: {left}px; top: {top}px;" class="draggable">
	{@render children()}
</section>

<svelte:window onmouseup={onMouseUp} on:mousemove={onMouseMove} />

<style lang="scss">
	.draggable {
		user-select: none;
		cursor: move;
		position: absolute;
	}
</style>
