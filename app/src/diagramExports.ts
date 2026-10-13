import Konva from 'konva';
import {PDFDocument,rgb} from 'pdf-lib';
import fontkit from '@pdf-lib/fontkit';
import {scene,fontBytes,fontFamily} from './diagramScene';
import {box,type CaseSnapshot,type TreeRow} from './model';
import {xml} from './exportXml';
import {loadFont} from './fontAssets';
const base64=(bytes:Uint8Array)=>{let text='';for(let i=0;i<bytes.length;i+=8192)text+=String.fromCharCode(...bytes.subarray(i,i+8192));return btoa(text);};
export async function diagramExport(snapshot:CaseSnapshot,diagram:TreeRow,format:string,scale:number):Promise<Uint8Array>{
 return sceneExport(scene(snapshot,diagram),`${snapshot.title} ${diagram.title}`,format,scale);
}
export async function sceneExport(drawing:ReturnType<typeof scene>&{directed?:boolean},name:string,format:string,scale:number):Promise<Uint8Array>{
 await loadFont();
 if(format==='svg'){
  const parts=[`<?xml version="1.0" encoding="UTF-8"?><svg xmlns="http://www.w3.org/2000/svg" width="${drawing.width}" height="${drawing.height}" viewBox="${drawing.minX} ${drawing.minY} ${drawing.width} ${drawing.height}"><title>${xml(name)}</title><defs>${drawing.directed?'<marker id="incident-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M 0 0 L 8 4 L 0 8 Z" fill="#819589"/></marker>':''}<style>@font-face{font-family:'${fontFamily}';src:url(data:font/ttf;base64,${base64(fontBytes)})}</style></defs><rect x="${drawing.minX}" y="${drawing.minY}" width="${drawing.width}" height="${drawing.height}" fill="#ffffff"/>`];
  for(const p of drawing.lines)parts.push(`<path d="M ${p[0]} ${p[1]} L ${p[2]} ${p[3]}" fill="none" stroke="#819589" stroke-width="2"${drawing.directed?' marker-end="url(#incident-arrow)"':''}/>`);
  for(const item of drawing.nodes){const {node,p,fill,title,annotation}=item;parts.push(`<g data-entity-id="${xml(node.id)}" transform="translate(${p.x} ${p.y})">`);parts.push(node.kind==='top_event'?`<ellipse cx="85" cy="39" rx="85" ry="39" fill="${fill}" stroke="#9bab9f"/>`:`<rect width="170" height="78" rx="${node.kind==='barrier'?3:10}" fill="${fill}" stroke="#9bab9f"/>`);
   const text=(content:string,x:number,y:number,size:number,color:string)=>parts.push(`<text x="${x}" y="${y}" font-family="${fontFamily}" font-size="${size}" fill="${color}">${xml(content)}</text>`);
   text(node.kind.replaceAll('_',' ').toUpperCase(),10,18,9,'#587066');title.forEach((v,i)=>text(v,12,39+i*13,13,'#223d31'));annotation.forEach((v,i)=>text(v,8,93+i*10,10,'#566d60'));parts.push('</g>');
  }parts.push('</svg>');return new TextEncoder().encode(parts.join(''));
 }
 if(format==='png'){
  if(!Number.isInteger(scale)||scale<1||scale>4)throw Error('PNG scale must be 1 to 4');
  if(drawing.width*drawing.height*scale*scale>100_000_000)throw Error('Choose a lower PNG scale for this large diagram');
  const container=document.createElement('div');container.style.cssText='position:fixed;left:-100000px;top:0';document.body.append(container);
  const stage=new Konva.Stage({container,width:drawing.width,height:drawing.height});const layer=new Konva.Layer();stage.add(layer);layer.add(new Konva.Rect({width:drawing.width,height:drawing.height,fill:'#fff'}));const group=new Konva.Group({x:-drawing.minX,y:-drawing.minY});layer.add(group);
  try{drawing.lines.forEach(points=>group.add(drawing.directed?new Konva.Arrow({points,stroke:'#819589',fill:'#819589',strokeWidth:2,pointerLength:7,pointerWidth:6}):new Konva.Line({points,stroke:'#819589',strokeWidth:2})));for(const item of drawing.nodes){const {node,p,fill,title,annotation}=item;const g=new Konva.Group({x:p.x,y:p.y});group.add(g);g.add(node.kind==='top_event'?new Konva.Ellipse({x:85,y:39,radiusX:85,radiusY:39,fill,stroke:'#9bab9f'}):new Konva.Rect({width:170,height:78,cornerRadius:node.kind==='barrier'?3:10,fill,stroke:'#9bab9f'}));g.add(new Konva.Text({x:10,y:9,text:node.kind.replaceAll('_',' ').toUpperCase(),fontFamily,fontSize:9,fill:'#587066'}));g.add(new Konva.Text({x:12,y:26,text:title.join('\n'),fontFamily,fontSize:13,fill:'#223d31',wrap:'none'}));g.add(new Konva.Text({x:8,y:83,text:annotation.join('\n'),fontFamily,fontSize:10,fill:'#566d60',wrap:'none'}));}layer.draw();return Uint8Array.from(atob(stage.toDataURL({pixelRatio:scale}).split(',')[1]),c=>c.charCodeAt(0));}finally{stage.destroy();container.remove();}
 }
 const doc=await PDFDocument.create({updateMetadata:false});doc.registerFontkit(fontkit);const font=await doc.embedFont(fontBytes,{subset:true});const unit=.9,page=doc.addPage([drawing.width*unit,drawing.height*unit]);doc.setTitle(name);doc.setCreator('Barrier Atlas');
 const color=(v:string)=>rgb(parseInt(v.slice(1,3),16)/255,parseInt(v.slice(3,5),16)/255,parseInt(v.slice(5,7),16)/255),X=(x:number)=>(x-drawing.minX)*unit,Y=(y:number)=>(drawing.height-(y-drawing.minY))*unit;
 drawing.lines.forEach(p=>{const segments=[p];if(drawing.directed){const dx=p[2]-p[0],dy=p[3]-p[1],length=Math.hypot(dx,dy);if(length){const ux=dx/length,uy=dy/length;segments.push([p[2]-8*ux+4*uy,p[3]-8*uy-4*ux,p[2],p[3]],[p[2]-8*ux-4*uy,p[3]-8*uy+4*ux,p[2],p[3]]);}}for(const q of segments)page.drawLine({start:{x:X(q[0]),y:Y(q[1])},end:{x:X(q[2]),y:Y(q[3])},thickness:2*unit,color:color('#819589')});});
 for(const item of drawing.nodes){const {node,p,fill,title,annotation}=item;
  if(node.kind==='top_event')page.drawEllipse({x:X(p.x+85),y:Y(p.y+39),xScale:85*unit,yScale:39*unit,color:color(fill),borderColor:color('#9bab9f'),borderWidth:unit});
  else {const r=node.kind==='barrier'?3:10,w=170,h=78;page.drawSvgPath(`M ${r} 0 H ${w-r} Q ${w} 0 ${w} ${r} V ${h-r} Q ${w} ${h} ${w-r} ${h} H ${r} Q 0 ${h} 0 ${h-r} V ${r} Q 0 0 ${r} 0 Z`,{x:X(p.x),y:Y(p.y),scale:unit,color:color(fill),borderColor:color('#9bab9f'),borderWidth:1});}
  const text=(v:string,x:number,y:number,size:number,c:string)=>page.drawText(v,{x:X(p.x+x),y:Y(p.y+y),size:size*unit,font,color:color(c)});
  text(node.kind.replaceAll('_',' ').toUpperCase(),10,18,9,'#587066');title.forEach((v,i)=>text(v,12,39+i*13,13,'#223d31'));annotation.forEach((v,i)=>text(v,8,93+i*10,10,'#566d60'));
 }return doc.save({useObjectStreams:false});
}
