import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY } from '$env/static/public';
import { createClient } from '@supabase/supabase-js';

// Preflight request handler
export const OPTIONS: RequestHandler = async () => {
    console.log("🟡 OPTIONS request received");
    return new Response(null, {
        status: 204,
        headers: {
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Methods': 'POST, OPTIONS',
            'Access-Control-Allow-Headers': 'Content-Type, Accept, Authorization, Origin',
            'Access-Control-Max-Age': '86400' // 24 hours
        }
    });
};

export const POST: RequestHandler = async ({ request }) => {
    console.log("🔥 API /api/signin was HIT");
	console.log("Request headers:", Object.fromEntries(request.headers));
    console.log("Request method:", request.method);
    console.log("Request URL:", request.url);
    
    try {
        // Parse request body
        let body;
        try {
            body = await request.json();
            console.log("Request body parsed successfully");
        } catch (e) {
            console.error("Failed to parse request body:", e);
            return json({ success: false, message: "Invalid request format" }, { 
                status: 400,
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                }
            });
        }
        
        const { email, password } = body;
        
        if (!email || !password) {
            console.error("Missing email or password");
            return json({ success: false, message: "Email and password are required" }, { 
                status: 400,
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                }
            });
        }
        
        console.log(`Attempting to sign in user: ${email}`);
        
        // Initialize Supabase client
        const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY);
        
        // Attempt login
        const { error, data } = await supabase.auth.signInWithPassword({
            email,
            password
        });
        
        if (error) {
            console.error('Login error:', error.message);
            return json({ success: false, message: error.message }, { 
                status: 401,
                headers: {
                    'Access-Control-Allow-Origin': '*',
                    'Content-Type': 'application/json'
                }
            });
        }
        
        console.log("Login successful for user:", email);
        
        // Return successful response
        return json({
            success: true,
            message: "Authentication successful",
            session: data.session,
            user: data.user
        }, {
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Content-Type': 'application/json'
            }
        });
    } catch (err) {
        console.error("💥 Error in handler:", err);
        return json({ success: false, message: "Server Error" }, { 
            status: 500,
            headers: {
                'Access-Control-Allow-Origin': '*',
                'Content-Type': 'application/json'
            }
        });
    }
};

export const GET: RequestHandler = async () => {
	console.log("Test endpoint hit!");
	return json({ message: "API is working" });
  };	