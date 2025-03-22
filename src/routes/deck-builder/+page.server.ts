import type { Actions } from './$types'
import type { PageServerLoad } from '../catalog/$types'
import type { Database } from '$lib/database.types'; 
import { error, fail } from '@sveltejs/kit';
type Card = Database['public']['Tables']['cards']['Row'];
type Deck = Database['public']['Tables']['decks']['Row'];

export const load: PageServerLoad = async ({ locals: { supabase, user  } }) => {
  const { data: cards } = await supabase.from("cards").select("*");
  const { data: decks } = await supabase.from("decks").select("*").eq("owner", user?.id)
  
  const typedDecks = decks as Deck[] | null;
  const typedCards = cards as Card[] | null;

  return { cards: typedCards ?? [], decks: typedDecks ?? []}
}

export const actions: Actions = {
  createDeck: async ({ request, locals: { supabase } }) => {
    const formData = await request.formData();
    const deckJSON = formData.get('deck') as string;
    const deckIdInput = formData.get("deckId") as string;

    // Convert empty string to null, then cast to number | null
    const deckId = deckIdInput === "" ? null : Number(deckIdInput);

    let deck: number[];

    try {
      deck = JSON.parse(deckJSON);
      if (!Array.isArray(deck)) console.log("deck is not array")
    } catch (error) {
      console.log("invalid data " + error);
      return;
    }

    // Validate the deck
    if (deck.length < 1 || deck.length > 30) {
      return { success: false, error: "Deck must contain between 1 and 30 cards" };
    }

    // Check if all card IDs are valid
    const { data: validCards, error: cardError } = await supabase
      .from('cards')
      .select('id')
      .in('id', deck);

    if (cardError) {
      return { success: false, error: "Deck is completely fucked" };
    } 

    const validCardIds = validCards.map((card) => card.id);
    const invalidCardIds = deck.filter((cardId) => !validCardIds.includes(cardId));

    if (invalidCardIds.length > 0) {
      return { success: false, error: `Invalid card IDs: ${invalidCardIds.join(', ')}`};
    }

    type DeckPayload = {
      owner: string | undefined;
      cards: number[];
      id?: number;
    }

    const deckPayload: DeckPayload = {
      owner: (await supabase.auth.getUser()).data.user?.id, 
      cards: deck,
    };
    
    // if deck already exist send id aswell to update existing table
    if (deckId !== null) {
      deckPayload.id = deckId;
    }

    const { data: deckData, error: deckError } = await supabase
      .from('decks')
      .upsert([deckPayload])
      .select('*')
      .single();

    if (deckError || !deckData) {
      console.error(deckError)
      return { success: false, error: "Deck is completely fucked" + deckError };
    }

    return { success: true, newDeck: deckData as Deck };
  },
};