import type { IncantationEntity } from "./shared/bindings/IncantationEntity";
import type { MinionEntity } from "./shared/bindings/MinionEntity";

export const drag = $state({
	card: null as (MinionEntity | IncantationEntity) | null,
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
