import type { PageServerLoad } from './$types'

export const load: PageServerLoad = async ({ locals: { supabase, session } }) => {
  const { error } = await supabase.rpc('create_profile_if_not_exists');
    error ? console.error('Error creating profile:', error) : console.log('Profile checked/created successfully');

  const { data: profile } = await supabase
    .from('profiles')
    .select(`username`)
    .eq('user_id', session?.user.id)
    .single()

  return { session, profile }
}