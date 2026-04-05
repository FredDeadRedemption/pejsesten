import { getCards } from "$lib/shared/cards";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
  const cards = getCards();
  console.log("CARDS:" + cards);
  return { cards: cards };
};