import type { CardEntity } from '$lib/shared/types';

export const targeting = $state({
  active: false,
  card: null as CardEntity | null,
  cardIndex: null as number | null,
  hoveredTarget: null as string | null,
})

export const beginTargeting = (card: CardEntity, index: number) => {
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
