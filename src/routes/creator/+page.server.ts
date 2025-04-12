// src/routes/cards/+page.server.ts
import type { Actions } from './$types'
import { fail } from '@sveltejs/kit';


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





export const actions: Actions = {
  createCard: async ({ request, locals: { supabase }}) => {
    const formData = await request.formData();

    console.log("YEEEHAW")

    // Extract form data
    const name = formData.get('name') as string;
    const attack = parseInt(formData.get('attack') as string);
    const defence = parseInt(formData.get('defence') as string);
    const white = parseInt(formData.get('white') as string);
    const black = parseInt(formData.get('black') as string);
    const purple = parseInt(formData.get('purple') as string);
    const green = parseInt(formData.get('green') as string);
    const red = parseInt(formData.get('red') as string);
    const orange = parseInt(formData.get('orange') as string);
    const description = formData.get('description') as string;
    const imageFile = formData.get('image') as File;

    // Validate required fields
    if (!name || !attack || !defence) {
      return fail(400, { error: 'Name, attack, and defence are required' });
    }

    // Validate image file
    if (imageFile && imageFile.size > 0) {
      const allowedTypes = ['image/jpeg', 'image/png', 'image/gif'];
      if (!allowedTypes.includes(imageFile.type)) {
        return fail(400, { error: 'Invalid file type. Only JPEG, PNG, and GIF are allowed.' });
      }

      if (imageFile.size > 5 * 1024 * 1024) { // 5MB limit
        return fail(400, { error: 'File size exceeds the 5MB limit.' });
      }
    }

    try {
      let imageUrl = null;

      // Upload image to Supabase Storage if provided
      if (imageFile && imageFile.size > 0) {
        const fileExt = imageFile.name.split('.').pop();
        const fileName = `card-${name.replaceAll(" ", "-")}.${fileExt}`;
        const filePath = `cards/${fileName}`; // Store in a "cards" folder for organization

        const { data: uploadData, error} = await supabase.storage
          .from('card-images') // Your bucket name
          .upload(filePath, imageFile, {
            cacheControl: '3600', // Cache for 1 hour
            upsert: true // Do not overwrite existing files
          });

        if (error) {
          console.error('Upload error:', JSON.stringify(error, null, 2));
          throw error;
        }

        // Get the public URL of the uploaded image
        const { data: { publicUrl } } = supabase.storage
          .from('card-images')
          .getPublicUrl(uploadData.path);

        imageUrl = publicUrl;
      }

      // Insert the new card into the database
      const { data: cardData, error: insertError } = await supabase
        .from('cards')
        .insert([{ 
          name, 
          attack, 
          defence, 
          white,
          black,
          purple,
          green,
          red,
          orange,
          description, 
          image_url: imageUrl 
        }])
        .select(); // Use .select() to return the inserted data

      if (insertError) {
        console.log("insertt error: " + insertError)
        throw insertError;
      }

      return { success: true, card: cardData[0] };
    } catch (error) {
      console.error('Error creating card:', error);
      return fail(500, { error: 'Failed to create card. Please try again.' });
    }
  }
}