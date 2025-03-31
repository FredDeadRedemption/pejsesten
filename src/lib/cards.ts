import type { Database } from '$lib/database.types'; 
type Card = Database['public']['Tables']['cards']['Row'];

let cards: Card[] = [];

export const setCards = (allCards: Card[]) => {
    cards = allCards;
}

export const getCardByID = (id: number) => {
    return cards.find(card => card.id === id);
}