import type { IncantationEntity } from './shared/bindings/IncantationEntity';
import type { MinionEntity } from './shared/bindings/MinionEntity';

export const targeting = $state({
  active: false,
  card: null as (MinionEntity | IncantationEntity) | null,
  cardIndex: null as number | null,
  hoveredTarget: null as number | null,
})

export const beginTargeting = (card: MinionEntity | IncantationEntity, index: number) => {
  targeting.card = card;
  targeting.cardIndex = index;
  targeting.active = true;
}

export const endTargeting = () => {
  targeting.active = false;
  targeting.card = null;
  targeting.cardIndex = null;
  targeting.hoveredTarget = null;
}
