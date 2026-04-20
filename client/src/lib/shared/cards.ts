import type { IncantationCard } from "./bindings/IncantationCard";
import type { MinionCard } from "./bindings/MinionCard";
import { PUBLIC_SERVER_URL } from '$env/static/public';
import { dev } from "$app/environment";



export const getCards = async (): Promise<(MinionCard | IncantationCard)[]> => {
   const baseUrl = dev ? 'http://localhost:3000' : PUBLIC_SERVER_URL;
    
    const res = await fetch(`${baseUrl}/cards`);
    const data = await res.json();
    console.log('Fetched cards:', data);
    return data.map((c: any) => {
        if ('Minion' in c) return { ...c.Minion, type: 'minion' };
        if ('Incantation' in c) return { ...c.Incantation, type: 'incantation' };
    });
};