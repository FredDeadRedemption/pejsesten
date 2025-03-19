<script>
	export let left = 500;
	export let top = 500;

	let moving = false;

	const onMouseDown = () => {
		moving = true;
	};

	const onMouseMove = (/** @type {{ movementX: number; movementY: number; }} */ e) => {
		if (moving) {
			left += e.movementX;
			top += e.movementY;
		}
	};

	const onMouseUp = () => {
		moving = false;
	};
	// Sjovt libary til det samme: https://gsap.com/docs/v3/Plugins/Draggable/
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<section on:mousedown={onMouseDown} style="left: {left}px; top: {top}px;" class="draggable">
	<slot />
</section>

<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} />

<style lang="scss">
	.draggable {
		user-select: none;
		cursor: pointer;
		position: absolute;
	}
</style>
