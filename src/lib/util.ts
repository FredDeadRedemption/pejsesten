import { browser } from "$app/environment";

export const enterFullscreen = (divID: string) => {
  if(!browser) return;
  const elem = document.getElementById(divID);
  if (elem?.requestFullscreen) {
    elem.requestFullscreen();
  }
}

export const getRandomString = (length: number): string => {
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let result = '';
  const charactersLength = characters.length;
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

export const getRandomDeckName = () => {
  const names = [
    "Pixie's Rejected Tarot",
    "Thoth's Hangover Deck",
    "Rider-Waite Smackdown",
    "The Fool's Tax Return",
    "Tower Moment Deck",
    "Moonlight Malpractice",
    "Golden Dawn Interns",
    "Crowley's Coffee Order",
    "Satan's To-Do List",
    "DMT Elf encounter",
    "Demonology for Dummies"
  ];

  return names[Math.floor(Math.random() * names.length)];
}