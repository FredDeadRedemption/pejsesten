import type { PageServerLoad } from '../catalog/$types'

export const load: PageServerLoad = async ({ locals: { supabase } }) => {
  const { data: land_enums } = await supabase.from('land_enums').select('*');

  const { data: cards } = await supabase.from("cards").select("*");

  return { land_enums: land_enums ?? [], cards: cards ?? [] }
}