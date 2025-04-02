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

  html2canvas(element ?? new HTMLElement(), {
    useCORS: true, // Attempt to load cross-origin images as CORS
    allowTaint: true, // Allow tainted canvas (but won't be readable)
  }).then(canvas => {
      // Create a download link
      const link = document.createElement('a');
      link.download = filename || 'div-image.png';
      link.href = canvas.toDataURL('image/png');
      link.click();
  })};