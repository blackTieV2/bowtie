import { useEffect, useMemo, useRef, useState } from "react";
import { Stage, Layer, Group, Rect, Text, Line, Ellipse } from "react-konva";
import type Konva from "konva";
import {scene, fontFamily} from "./diagramScene";
import {useAuditDiagram,resultSegments} from "./AuditDiagram";
import type {Mutate} from "./RiskPanels";
import { box, classicLayout, diagramNodes, savedPositions, type CaseSnapshot, type Point, type TreeRow } from "./model";

type Props = { snapshot: CaseSnapshot; diagram: TreeRow; selected: string | null; busy: boolean; mutate:Mutate; select: (id: string) => void; edit: (node: TreeRow) => void; persist: (positions: {id:string;x:number;y:number}[]) => Promise<boolean> };
export default function DiagramCanvas({snapshot, diagram, selected, busy, mutate, select, edit, persist}: Props) {
  const host = useRef<HTMLDivElement>(null), stage = useRef<Konva.Stage>(null);
  const [size,setSize] = useState({width:800,height:550});
  const [view,setView] = useState({x:15,y:15,scale:0.6});
  const [moving,setMoving] = useState<Record<string,Point>>({});
  const nodes = useMemo(() => diagramNodes(snapshot.tree,diagram),[snapshot.tree,diagram]);
  const positions = {...savedPositions(nodes),...moving};
  const audit=useAuditDiagram(snapshot,nodes.filter(n=>n.kind==='barrier').map(n=>n.id),mutate,busy);
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
  const drawing=scene(snapshot,diagram,positions,audit.questionAnnotations);
  return <section className="diagram-area" aria-label="Diagram canvas">
    {audit.controls}
    {audit.dialog}
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
          {drawing.lines.map((points,i)=><Line key={i} points={points} stroke="#819589" strokeWidth={2} listening={false}/>)}
          {drawing.nodes.map(({node,p,fill,title,annotation,questionLines})=>{
            return <Group key={node.id} id={node.id} x={p.x} y={p.y} draggable={!busy} onClick={()=>select(node.id)} onTap={()=>select(node.id)} onDblClick={()=>edit(node)} onDblTap={()=>edit(node)}
              onDragStart={e=>{e.cancelBubble=true;select(node.id);}}
              onDragMove={e=>{e.cancelBubble=true;setMoving(m=>({...m,[node.id]:e.target.position()}));}}
              onDragEnd={e=>{e.cancelBubble=true;const target=e.target;const next=target.position();void persist([{id:node.id,...next}]).then(ok=>{if(!ok){target.position(savedPositions(nodes)[node.id]);setMoving({});}});}}>
              {node.kind==="top_event" ? <Ellipse x={box.width/2} y={box.height/2} radiusX={box.width/2} radiusY={box.height/2} fill={fill} stroke={selected===node.id?"#245b47":"#9bab9f"} strokeWidth={selected===node.id?3:1}/> : <Rect width={box.width} height={box.height} cornerRadius={node.kind==="barrier"?3:10} fill={fill} stroke={selected===node.id?"#245b47":"#9bab9f"} strokeWidth={selected===node.id?3:1}/>}
              <Text fontFamily={fontFamily} x={10} y={9} width={box.width-20} text={node.kind.replaceAll("_"," ").toUpperCase()} fontSize={9} fill="#587066" listening={false}/>
              <Text fontFamily={fontFamily} wrap="none" x={12} y={26} width={box.width-24} height={43} text={title.join("\n")} fontSize={13} fill="#223d31" ellipsis listening={false}/>
              {questionLines.length>0&&<Text fontFamily={fontFamily} x={8} y={box.height+5+(audit.values[node.id]?14:0)} width={box.width} text={questionLines.join("\n")} fontSize={9} wrap="none" fill="#345f78"/>}
              {annotation.length>0 && <Text fontFamily={fontFamily} x={8} y={box.height+5+(audit.values[node.id]?14:0)+questionLines.length*10} width={box.width} text={annotation.join("\n")} fontSize={10} height={42} wrap="none" fill="#566d60" listening={false}/>}
              {audit.values[node.id]&&(()=>{const r=audit.values[node.id],counts=resultSegments(r),total=counts.positive+counts.neutral+counts.negative;let x=4;return <Group onClick={e=>{e.cancelBubble=true;audit.openDetails(node.id);}}>{(['negative','neutral','positive'] as const).map(key=>{const width=total?counts[key]/total*(box.width-8):0;const segment=<Rect key={key} x={x} y={70} width={width} height={5} fill={key==='positive'?'#477b9d':key==='negative'?'#a45b34':'#d8d5ce'}/>;x+=width;return segment;})}<Text fontFamily={fontFamily} x={4} y={box.height+2} width={box.width} fontSize={8} text={total?r.mostNegative?`Audit worst: ${r.choices[r.worstChoice!].label} · n=${r.counts.reduce((a,b)=>a+b,0)}`:`Audit −${counts.negative} / ○${counts.neutral} / +${counts.positive}`:'Audit: no answers'}/></Group>;})()}
            </Group>;
          })}
        </Layer>
      </Stage>
    </div>
  </section>;
}
