import { browser } from "$app/environment";
import html2canvas from "html2canvas";

export const enterFullscreen = (divID: string) => {
  if(!browser) return;
  const elem = document.getElementById(divID);
  if (elem?.requestFullscreen) {
    elem.requestFullscreen();
  }
}

export const downloadDivAsPNG = (divId: string, filename: string) => {
  if(!browser) return;
  const element = document.getElementById(divId);
  if(!element) return;

  filename = filename.toLowerCase().replace(" ", "_");

  html2canvas(element, {
    useCORS: true, // Attempt to load cross-origin images as CORS
    allowTaint: true, // Allow tainted canvas (but won't be readable)
  }).then(canvas => {
      // Create a download link
      const link = document.createElement('a');
      link.download = filename || 'div-image.png';
      link.href = canvas.toDataURL('image/png');
      link.click();
  })
};

export const getRandomString = () => Math.random().toString(36).slice(2, 12);

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