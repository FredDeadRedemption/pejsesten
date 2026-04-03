import { browser } from "$app/environment";

export const enterFullscreen = (divID: string) => {
  if(!browser) return;
  const elem = document.getElementById(divID);
  if (elem?.requestFullscreen) {
    elem.requestFullscreen();
  }
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