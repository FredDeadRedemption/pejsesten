export const drag = $state({
	index: null as number | null,
	x: 0,
	y: 0,
	returning: false,
	consumed: false
});

export const resetDrag = () => {
	drag.consumed = false;
	drag.index = null;
};
