import type { IncantationCard } from "./bindings/IncantationCard";
import type { MinionCard } from "./bindings/MinionCard";

export const getCards = async (): Promise<(MinionCard | IncantationCard)[]> => {
    const res = await fetch('http://localhost:3000/cards');
    const data = await res.json();
    return data.map((c: any) => {
        if ('Minion' in c) return { ...c.Minion, type: 'minion' };
        if ('Incantation' in c) return { ...c.Incantation, type: 'incantation' };
    });
};