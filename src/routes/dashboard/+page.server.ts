import type { PageServerLoad } from './$types'

export const load: PageServerLoad = async ({ locals: { supabase } }) => {
  const { data: land_enums } = await supabase.from('land_enums').select('*');
  return { land_enums: land_enums ?? [] }
}