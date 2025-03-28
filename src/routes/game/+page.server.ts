import type { PageServerLoad } from '../catalog/$types'
import type { Database } from '$lib/database.types'; 
type Deck = Database['public']['Tables']['decks']['Row'];

export const load: PageServerLoad = async ({ locals: { supabase, user } }) => {
  const { data: decks } = await supabase.from("decks").select("*").eq("owner", user?.id)
  
  const typedDecks = decks as Deck[] | null;

  return { decks: typedDecks ?? []}
}