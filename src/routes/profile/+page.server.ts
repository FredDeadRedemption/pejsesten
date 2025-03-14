import { fail, redirect } from '@sveltejs/kit'
import type { Actions, PageServerLoad } from './$types'

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

export const actions: Actions = {
  update: async ({ request, locals: { supabase, safeGetSession } }) => {
    const formData = await request.formData()
    const username = formData.get('username') as string

    const { session } = await safeGetSession()

    const { error } = await supabase.from('profiles')
      .update({
        username,
        updated_at: new Date()
      })
      .eq('user_id', session?.user.id); 

    if (error) {
      console.log("faiiled")
      return fail(500, {
        username
      })
    }

    console.log("updated username")

    return {
      username
    }
  }
}
