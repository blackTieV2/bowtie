import {wrap,type SceneNode} from './diagramScene';
import {type TreeRow} from './model';
import {causeLabels,timelineLabel,timelineRows,type Incident,type IncidentDiagram} from './incidentModel';
export const incidentBox={width:170,height:78};
export function incidentScene(a:Incident,d:IncidentDiagram){
 const nodes:SceneNode[]=[],lines:number[][]=[],rows=timelineRows(a,d);
 const add=(id:string,kind:string,title:string,x:number,y:number,fill:string,annotation='',ownerId=id)=>{const node:TreeRow={id,kind,title,parentId:null,sortOrder:0,depth:0,description:'',properties:{incidentNodeId:ownerId},effectivenessId:null};nodes.push({node,p:{x,y},fill,title:wrap(title,13,146,3),annotation:wrap(annotation,10,170,6),questionLines:[]});};
 if(d.timeline){d.columns.forEach((c,i)=>add(c.id,'time column',timelineLabel(c),190+i*210,10,c.color));rows.forEach(row=>add(row.id??'unassigned','actor',row.title,0,row.y,a.nodes.find(n=>n.id===row.id)?.metadata.color??'#edf2ed'));
 for(const n of a.nodes.filter(n=>n.metadata.diagramId===d.id&&n.kind!=='actor')){const column=d.columns.findIndex(c=>c.id===n.metadata.columnId),row=rows.find(v=>v.id===n.metadata.actorId)??rows[rows.length-1],same=a.nodes.filter(v=>v.kind!=='actor'&&v.metadata.diagramId===d.id&&v.metadata.columnId===n.metadata.columnId&&v.metadata.actorId===n.metadata.actorId).sort((a,b)=>a.id.localeCompare(b.id)),slot=same.findIndex(v=>v.id===n.id);add(n.id,n.kind,n.title,190+column*210,row.y+slot*160,n.metadata.color,[n.metadata.location,n.metadata.confidence].filter(Boolean).join(' · '));}
 }else{
 if(a.methodKey==='categorized_rca')d.zones.forEach((label,i)=>add(`region:${d.id}:${i}`,'analysis region',label||`Region ${['1A','1B','2','3','4'][i]}`,i*200,-130,'#ecefe9'));
 for(const n of a.nodes.filter(n=>n.metadata.diagramId===d.id)) {const m=n.metadata;add(n.id,n.kind==='event'?m.appearance:n.kind,n.title,m.x,m.y,m.color,n.kind==='barrier'?`${m.state} · ${m.failureShort}`:m.description);n.metadata.causes.forEach((c,ci)=>{c.texts.forEach((text,level)=>{const cid=`cause:${n.id}:${c.id}:${level}`,x=m.x+level*200,y=m.y+140+ci*160;add(cid,causeLabels(a)[level],text||'Not described',x,y,['#f1e1dc','#e9e4cd','#dce6ed'][level],c.categories[level].map(id=>a.metadata.taxonomy.find(t=>t.id===id)?.title??'').join(' · '),n.id);if(level===0)lines.push([m.x+85,m.y+78,x+85,y]);else lines.push([x-30,y+39,x,y+39]);});});}
 for(const e of a.edges){const s=nodes.find(n=>n.node.id===e.sourceId),t=nodes.find(n=>n.node.id===e.targetId);if(s&&t)lines.push([s.p.x+170,s.p.y+39,t.p.x,t.p.y+39]);}
 }
 const minX=Math.min(0,...nodes.map(n=>n.p.x))-30,minY=Math.min(0,...nodes.map(n=>n.p.y))-30,maxX=Math.max(180,...nodes.map(n=>n.p.x+170))+30,maxY=Math.max(100,...nodes.map(n=>n.p.y+85+n.annotation.length*10))+30;
 return{directed:true,nodes,lines,minX,minY,width:Math.ceil(maxX-minX),height:Math.ceil(maxY-minY)};
}
