import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { createClient } from '@supabase/supabase-js';
import { PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY } from '$env/static/public';

export const OPTIONS: RequestHandler = async () => {
    console.log("🟡 OPTIONS request received for /api/getalldecks");
    return new Response(null, {
        status: 204,
        headers: {
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Methods': 'GET, OPTIONS',
            'Access-Control-Allow-Headers': 'Content-Type, Accept, Authorization, Origin',
        }
    });
};

export const GET: RequestHandler = async ({ request }) => {
    console.log("🔥 API /api/getalldecks was HIT");
    
    // Get the authorization header
    const authHeader = request.headers.get('Authorization');
    
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
        console.error("No valid authorization header found");
        return json({ 
            success: false, 
            message: "Unauthorized" 
        }, { 
            status: 401,
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Content-Type': 'application/json'
            }
        });
    }
    
    // Extract the token
    const token = authHeader.split(' ')[1];
    
    try {
        // Initialize Supabase client
        const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY);
        
        // Set the auth token for this client instance
        const { data: { user }, error: authError } = await supabase.auth.getUser(token);
        
        if (authError || !user) {
            console.error('Authentication error:', authError?.message || "User not found");
            return json({ 
                success: false, 
                message: authError?.message || "User not found" 
            }, { 
                status: 401,
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                }
            });
        }
        
        // Fetch additional profile information if you have a profiles table
        // This is optional and depends on your Supabase setup
        const { data: decks, error: decksError } = await supabase
            .from('decks')  
            .select('id, name, cards')
            .eq('owner', user.id);

        if (decksError) {
            console.log('Decks fetch error:', decksError.message);
            // Continue anyway, we'll just return the basic user info
        }

        
        // Return user data and profile if available
        return json({
            success: true,
            decks: decks || null
        }, {
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Content-Type': 'application/json'
            }
        });
    } catch (err) {
        console.error("💥 Error in handler:", err);
        return json({ 
            success: false, 
            message: "Server Error" 
        }, { 
            status: 500,
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Content-Type': 'application/json'
            }
        });
    }
};