import type { CardEntity } from '$lib/shared/types';

export const drag = $state({
	card: null as CardEntity | null,
	index: null as number | null,
	x: 0,
	y: 0,
	returning: false,
	consumed: false,
});