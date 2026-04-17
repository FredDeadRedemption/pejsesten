import type { Card } from "./bindings/Card";

export const getCards = async (): Promise<Card[]> => {
    const res = await fetch('http://localhost:3000/cards');
    return res.json();
};