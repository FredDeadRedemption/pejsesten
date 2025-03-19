import type { Actions } from './$types'
import type { PageServerLoad } from '../catalog/$types'
import type { Database } from '$lib/database.types'; 
type Card = Database['public']['Tables']['cards']['Row'];

export const load: PageServerLoad = async ({ locals: { supabase } }) => {
  const { data: cards } = await supabase.from("cards").select("*");

  const typedCards = cards as Card[] | null;

  return { cards: typedCards ?? [] }
}

export const actions: Actions = {
  createDeck: async({ request, locals: { supabase } }) =>{
    const formData = await request.formData();
    const deck = formData.get('deck') as string;
    console.log(deck)
  } 

}