import type { LayoutServerLoad } from './$types'

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
    .select('username')
    .eq('user_id', session?.user.id)
    .single();
    
  console.log(profile)

  return {
    profile,
    session,
    cookies: cookies.getAll(),
  }
}