import fontkit from '@pdf-lib/fontkit';
import {box,diagramNodes,savedPositions,type CaseSnapshot,type TreeRow,type Point} from './model';
import {fontBytes,fontFamily} from './fontAssets';
export {fontBytes,fontFamily} from './fontAssets';
const font=fontkit.create(fontBytes);
export const textWidth=(text:string,size:number)=>font.layout(text).positions.reduce((sum:number,p:{xAdvance:number})=>sum+p.xAdvance,0)*size/font.unitsPerEm;
export function wrap(text:string,size:number,width:number,maxLines:number):string[]{
 const words=text.replace(/\s+/g,' ').trim().split(' '),lines:string[]=[];let line='';
 for(const word of words){let pending=word;const next=line?`${line} ${pending}`:pending;if(textWidth(next,size)<=width){line=next;continue;}if(line){lines.push(line);line='';}
 while(textWidth(pending,size)>width && pending.length>1){let cut=pending.length;while(cut>1 && textWidth(pending.slice(0,cut),size)>width)cut--;lines.push(pending.slice(0,cut));pending=pending.slice(cut);}line=pending;}
 if(line)lines.push(line);if(lines.length>maxLines){lines.length=maxLines;let last=lines[maxLines-1];while(last && textWidth(last+'…',size)>width)last=last.slice(0,-1);lines[maxLines-1]=last+'…';}return lines;
}
export const colors:Record<string,string>={hazard:'#e9dcc6',top_event:'#f0c9c1',threat:'#d8e8ee',consequence:'#f6dfba',barrier:'#d9e9d9',escalation_factor:'#e5dcf1'};
export type SceneNode={node:TreeRow;p:Point;fill:string;title:string[];annotation:string[]};
export function scene(snapshot:CaseSnapshot,diagram:TreeRow,positionsOverride?:Record<string,Point>){
 const nodes=diagramNodes(snapshot.tree,diagram),positions=positionsOverride??savedPositions(nodes),edges:[string,string][]=[];
 const top=nodes.find(n=>n.kind==='top_event'),hazard=nodes.find(n=>n.kind==='hazard');if(top&&hazard)edges.push([hazard.id,top.id]);
 for(const path of nodes.filter(n=>['threat','consequence','escalation_factor'].includes(n.kind))){const barriers=nodes.filter(n=>n.kind==='barrier'&&n.parentId===path.id).sort((a,b)=>a.sortOrder-b.sortOrder||a.id.localeCompare(b.id));const chain=[path.id,...barriers.map(b=>b.id),path.parentId!];for(let i=1;i<chain.length;i++)edges.push([chain[i-1],chain[i]]);}
 const lines=edges.flatMap(([a,b])=>{const from=positions[a],to=positions[b];if(!from||!to)return [];const dx=to.x-from.x,dy=to.y-from.y,f=Math.max(Math.abs(dx)/(box.width/2),Math.abs(dy)/(box.height/2)),inset=f>0?1/f:0;return [[from.x+box.width/2+dx*inset,from.y+box.height/2+dy*inset,to.x+box.width/2-dx*inset,to.y+box.height/2-dy*inset]];});
 const drawn:SceneNode[]=nodes.filter(n=>positions[n.id]).map(node=>{
 const effect=snapshot.lookups.find(l=>l.id===node.effectivenessId),links=snapshot.links.filter(l=>l.sourceId===node.id),actions=snapshot.tree.filter(n=>n.kind==='action'&&n.parentId===node.id);
 const annotation=[...(effect?[`Effectiveness: ${effect.title}`]:[]),...links.map(l=>snapshot.tree.find(n=>n.id===l.targetId)?.title??''),...actions.map(n=>`Action: ${n.properties.code||n.title}`)].join(' · ');
 return {node,p:positions[node.id],fill:effect?.color??colors[node.kind],title:wrap(node.title,13,box.width-24,3),annotation:wrap(annotation,10,box.width,4)};
 });
 const minX=Math.min(...drawn.map(n=>n.p.x))-30,minY=Math.min(...drawn.map(n=>n.p.y))-30,maxX=Math.max(...drawn.map(n=>n.p.x+box.width))+30,maxY=Math.max(...drawn.map(n=>n.p.y+box.height+5+n.annotation.length*10))+30;
 return {nodes:drawn,lines,minX,minY,width:Math.ceil(maxX-minX),height:Math.ceil(maxY-minY)};
}
