// $lib/stores/targeting.ts
import type { CardEntity } from '$lib/shared/types';

type TargetingState = {
  active: boolean
  card: CardEntity | null
  cardIndex: number | null
}

export const targeting = $state<TargetingState>({
  active: false,
  card: null,
  cardIndex: null,
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
}