import type { LayoutServerLoad } from './$types'
import type { Database } from '$lib/database.types';
type Profile = Database['public']['Tables']['profiles']['Row'];

export const load: LayoutServerLoad = async ({ locals: { supabase, safeGetSession }, cookies }) => {
  const { session } = await safeGetSession()

  const { error } = await supabase.rpc('create_profile_if_not_exists');
  if (error) {
    console.error('Error creating profile:', error);
  } else {
    console.log('Profile checked/created successfully');
 }

  // Fetch the profile data
  const { data: profile } = await supabase
    .from('profiles')
    .select('username, avatar_url, is_admin')
    .eq('user_id', session?.user.id)
    .single<Profile>();
    
  console.log(profile)

  return {
    profile,
    session,
    cookies: cookies.getAll(),
  }
}