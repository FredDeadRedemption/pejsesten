import { getCards } from "$lib/shared/cards";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
  return { cards: getCards() };
};