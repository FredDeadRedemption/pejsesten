import type { IncantationCard } from "./bindings/IncantationCard";
import type { MinionCard } from "./bindings/MinionCard";
import { PUBLIC_SERVER_URL } from '$env/static/public';

export const getCards = async (): Promise<(MinionCard | IncantationCard)[]> => {
    const res = await fetch(`${PUBLIC_SERVER_URL}/cards`);
    const data = await res.json();
    return data.map((c: any) => {
        if ('Minion' in c) return { ...c.Minion, type: 'minion' };
        if ('Incantation' in c) return { ...c.Incantation, type: 'incantation' };
    });
};