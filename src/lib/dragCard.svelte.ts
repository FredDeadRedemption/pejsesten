// $lib/drag.svelte.ts
import type { CardEntity } from '$lib/shared/types';

export const drag = $state({
	card: null as CardEntity | null,
	index: null as number | null,
	x: 0,
	y: 0,
	returning: false
});

export const beginDrag = (card: CardEntity, index: number, x: number, y: number) => {
	drag.card = card;
	drag.index = index;
	drag.x = x;
	drag.y = y;
	drag.returning = false;
};

export const endDrag = () => {
	drag.card = null;
	drag.index = null;
	drag.returning = false;
};
