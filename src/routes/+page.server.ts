import { redirect, fail } from '@sveltejs/kit'

import type { Actions } from './auth/$types'

export const actions: Actions = {
  signup: async ({ request, locals: { supabase } }) => {
    const formData = await request.formData()
    const email = formData.get('email') as string
    const password = formData.get('password') as string

    const { error } = await supabase.auth.signUp({ email, password })
    if (error) {
      console.error(error)
      return { success: false, message: error.message};
    } else {
      return { success: true, message: "Account created - please check your mail"};
    }
  },
  login: async ({ request, locals: { supabase } }) => {
    const formData = await request.formData()
    const email = formData.get('email') as string
    const password = formData.get('password') as string

    const { error } = await supabase.auth.signInWithPassword({ email, password })
    if (error) {
      console.error(error)
      return { success: false, message: error.message};
    } else {
      redirect(303, '/dashboard?refresh=true')
    }
  },
}