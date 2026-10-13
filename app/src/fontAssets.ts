import fontDataUrl from './assets/NotoSans-Regular.ttf?inline';
export const fontBytes=Uint8Array.from(atob(fontDataUrl.split(',')[1]),c=>c.charCodeAt(0));
export const fontFamily='Barrier Atlas Sans';
let ready:Promise<void>|undefined;
export function loadFont(){return ready??=(async()=>{const face=new FontFace(fontFamily,fontBytes.slice().buffer);await face.load();document.fonts.add(face);await document.fonts.ready;})();}
