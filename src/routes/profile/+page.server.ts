import { fail } from '@sveltejs/kit'
import type { Actions } from './$types'

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
