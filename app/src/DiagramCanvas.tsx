import { useEffect, useMemo, useRef, useState } from "react";
import { Stage, Layer, Group, Rect, Text, Line, Ellipse } from "react-konva";
import type Konva from "konva";
import { box, classicLayout, diagramNodes, savedPositions, type CaseSnapshot, type Point, type TreeRow } from "./model";

type Props = { snapshot: CaseSnapshot; diagram: TreeRow; selected: string | null; busy: boolean; select: (id: string) => void; edit: (node: TreeRow) => void; persist: (positions: {id:string;x:number;y:number}[]) => Promise<boolean> };
export default function DiagramCanvas({snapshot, diagram, selected, busy, select, edit, persist}: Props) {
  const host = useRef<HTMLDivElement>(null), stage = useRef<Konva.Stage>(null);
  const [size,setSize] = useState({width:800,height:550});
  const [view,setView] = useState({x:15,y:15,scale:0.6});
  const [moving,setMoving] = useState<Record<string,Point>>({});
  const nodes = useMemo(() => diagramNodes(snapshot.tree,diagram),[snapshot.tree,diagram]);
  const positions = {...savedPositions(nodes),...moving};
  useEffect(() => { setMoving({}); },[snapshot]);
  useEffect(() => {
    const observer = new ResizeObserver(entries => { const rect=entries[0].contentRect; setSize({width:rect.width,height:Math.max(380,rect.height)}); });
    if (host.current) observer.observe(host.current);
    return () => observer.disconnect();
  },[]);
  const fit = (coords = positions) => {
    const points = Object.values(coords);
    if (!points.length) return;
    const left=Math.min(...points.map(p=>p.x)),top=Math.min(...points.map(p=>p.y));
    const width=Math.max(...points.map(p=>p.x))+box.width-left,height=Math.max(...points.map(p=>p.y))+box.height-top;
    const scale=Math.max(.05,Math.min(1,(size.width-70)/width,(size.height-70)/height));
    setView({x:(size.width-width*scale)/2-left*scale,y:(size.height-height*scale)/2-top*scale,scale});
  };
  useEffect(() => { fit(savedPositions(nodes)); },[diagram.id,size.width,size.height]); // adding entities preserves the user's view
  function zoom(factor:number, pointer={x:size.width/2,y:size.height/2}) {
    setView(old=> { const scale=Math.max(.05,Math.min(3,old.scale*factor)); return {scale,x:pointer.x-(pointer.x-old.x)*scale/old.scale,y:pointer.y-(pointer.y-old.y)*scale/old.scale}; });
  }
  async function autoLayout() {
    const layout=classicLayout(nodes);
    if (await persist(Object.entries(layout).map(([id,p])=>({id,...p})))) fit(layout);
  }
  const edges: [string,string][]=[];
  const top=nodes.find(n=>n.kind==="top_event");
  const hazard=nodes.find(n=>n.kind==="hazard");
  if (top && hazard) edges.push([hazard.id,top.id]);
  for (const path of nodes.filter(n=>["threat","consequence","escalation_factor"].includes(n.kind))) {
    const barriers=nodes.filter(n=>n.kind==="barrier" && n.parentId===path.id).sort((a,b)=>a.sortOrder-b.sortOrder || a.id.localeCompare(b.id));
    const chain=[path.id,...barriers.map(b=>b.id),path.parentId!];
    for(let i=1;i<chain.length;i++) edges.push([chain[i-1],chain[i]]);
  }
  const color:Record<string,string>={hazard:"#e9dcc6",top_event:"#f0c9c1",threat:"#d8e8ee",consequence:"#f6dfba",barrier:"#d9e9d9",escalation_factor:"#e5dcf1"};
  return <section className="diagram-area" aria-label="Diagram canvas">
    <div className="canvas-tools"><button disabled={busy} onClick={()=>void autoLayout()}>Auto-layout</button><button onClick={()=>fit()}>Fit</button><button aria-label="Zoom in" onClick={()=>zoom(1.2)}>＋</button><button aria-label="Zoom out" onClick={()=>zoom(1/1.2)}>−</button><button onClick={()=>setView({x:15,y:15,scale:1})}>100%</button><output aria-label="Canvas zoom">{Math.round(view.scale*100)}%</output><span>Drag shapes to move · Drag background to pan · Ctrl + wheel to zoom</span></div>
    <div ref={host} className="canvas-host" tabIndex={0} aria-label="Interactive diagram" data-view-x={view.x} data-view-y={view.y} data-view-scale={view.scale} onKeyDown={event=>{
      const delta=event.ctrlKey?200:35;
      const movement:Record<string,Point>={ArrowLeft:{x:delta,y:0},ArrowRight:{x:-delta,y:0},ArrowUp:{x:0,y:delta},ArrowDown:{x:0,y:-delta}};
      const p=movement[event.key]; if(p){event.preventDefault();setView(v=>({...v,x:v.x+p.x,y:v.y+p.y}));}
    }}>
      <Stage ref={stage} width={size.width} height={size.height} x={view.x} y={view.y} scaleX={view.scale} scaleY={view.scale} draggable={!busy}
        onDragEnd={e=>{if(e.target===stage.current)setView(v=>({...v,x:e.target.x(),y:e.target.y()}));}}
        onWheel={e=>{e.evt.preventDefault();if(e.evt.ctrlKey)zoom(e.evt.deltaY>0?1/1.1:1.1,stage.current?.getPointerPosition()??undefined);else setView(v=>({...v,x:v.x-(e.evt.shiftKey?e.evt.deltaY:e.evt.deltaX),y:v.y-(e.evt.shiftKey?0:e.evt.deltaY)}));}}>
        <Layer>
          {edges.map(([a,b])=>{
            const from=positions[a],to=positions[b]; if(!from || !to)return null;
            const center=(p:Point)=>({x:p.x+box.width/2,y:p.y+box.height/2});
            const start=center(from),end=center(to),dx=end.x-start.x,dy=end.y-start.y;
            const fraction=Math.max(Math.abs(dx)/(box.width/2),Math.abs(dy)/(box.height/2));
            const inset=fraction>0?1/fraction:0;
            return <Line key={`${a}/${b}`} points={[start.x+dx*inset,start.y+dy*inset,end.x-dx*inset,end.y-dy*inset]} stroke="#819589" strokeWidth={2} listening={false}/>;
          })}
          {nodes.map(node=>{
            const p=positions[node.id]; if(!p)return null;
            const links=snapshot.links.filter(l=>l.sourceId===node.id);
            const actions=snapshot.tree.filter(n=>n.kind==="action" && n.parentId===node.id);
            return <Group key={node.id} id={node.id} x={p.x} y={p.y} draggable={!busy} onClick={()=>select(node.id)} onTap={()=>select(node.id)} onDblClick={()=>edit(node)} onDblTap={()=>edit(node)}
              onDragStart={e=>{e.cancelBubble=true;select(node.id);}}
              onDragMove={e=>{e.cancelBubble=true;setMoving(m=>({...m,[node.id]:e.target.position()}));}}
              onDragEnd={e=>{e.cancelBubble=true;const target=e.target;const next=target.position();void persist([{id:node.id,...next}]).then(ok=>{if(!ok){target.position(savedPositions(nodes)[node.id]);setMoving({});}});}}>
              {node.kind==="top_event" ? <Ellipse x={box.width/2} y={box.height/2} radiusX={box.width/2} radiusY={box.height/2} fill={color[node.kind]} stroke={selected===node.id?"#245b47":"#9bab9f"} strokeWidth={selected===node.id?3:1}/> : <Rect width={box.width} height={box.height} cornerRadius={node.kind==="barrier"?3:10} fill={color[node.kind]} stroke={selected===node.id?"#245b47":"#9bab9f"} strokeWidth={selected===node.id?3:1}/>}
              <Text x={10} y={9} width={box.width-20} text={node.kind.replaceAll("_"," ").toUpperCase()} fontSize={9} fill="#587066" listening={false}/>
              <Text x={12} y={26} width={box.width-24} height={43} text={node.title} fontSize={13} fill="#223d31" ellipsis listening={false}/>
              {(links.length>0 || actions.length>0) && <Text x={8} y={box.height+5} width={box.width} text={[...links.map(l=>snapshot.tree.find(n=>n.id===l.targetId)?.title??""),...actions.map(n=>`Action: ${n.title}`)].join(" · ")} fontSize={10} height={36} ellipsis fill="#566d60" listening={false}/>}
            </Group>;
          })}
        </Layer>
      </Stage>
    </div>
  </section>;
}
