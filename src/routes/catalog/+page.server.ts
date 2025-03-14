import type { PageServerLoad } from '../catalog/$types'
import type { Database } from '$lib/database.types'; 
type LandEnum = Database['public']['Tables']['land_enums']['Row'];
type Card = Database['public']['Tables']['cards']['Row'];

export const load: PageServerLoad = async ({ locals: { supabase } }) => {
  const { data: land_enums } = await supabase.from('land_enums').select('*');

  const { data: cards } = await supabase.from("cards").select("*");

  const typedLandEnums = land_enums as LandEnum[] | null;
  const typedCards = cards as Card[] | null;

  return { land_enums: typedLandEnums ?? [], cards: typedCards ?? [] }
}