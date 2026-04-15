import type { CardEntity } from '$lib/shared/types';

export const drag = $state({
	card: null as CardEntity | null,
	index: null as number | null,
	x: 0,
	y: 0,
	returning: false,
	consumed: false
});

export const resetDrag = () => {
	drag.consumed = false;
	drag.card = null;
	drag.index = null;
};
