import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { label, type CaseSnapshot, type TreeRow } from "./model";
import { CustomFields, PropertyFields, fieldsFor, relations } from "./PropertyFields";
export type EditRequest = { node?:TreeRow; parentId:string; kind:string };
type Props = { request:EditRequest; busy:boolean; error:string; close:()=>void; mutate:(action:()=>Promise<CaseSnapshot>)=>Promise<boolean>; snapshot:CaseSnapshot;inline?:boolean };
export default function EntityEditor({request,busy,error,close,mutate,snapshot,inline}:Props) {
  const [title,setTitle]=useState(request.node?.title??"");
  const [description,setDescription]=useState(request.node?.description??"");
  const [properties,setProperties]=useState<Record<string,unknown>>(()=>({
    ...Object.fromEntries(fieldsFor(request.kind).map(f=>[f.key,request.node?.properties[f.key]??(f.type==='checkbox'?false:f.type==='integer'?null:'')])),
    ...(request.kind==='barrier'?{effectiveness_id:request.node?.effectivenessId??''}:{}),
    user_data:request.node?.properties.user_data??{},
    links:Object.fromEntries((relations[request.kind]??[]).map(r=>[r.relation,snapshot.links.filter(l=>l.sourceId===request.node?.id && l.relation===r.relation).map(l=>l.targetId)]))
  }));
  const set=(key:string,value:unknown)=>setProperties(p=>({...p,[key]:value}));
  async function submit() {
    const command=request.node?"case_edit_entity":"case_add_entity";
    const args=request.node?{id:request.node.id,title,description,properties}:{parentId:request.parentId,kind:request.kind,title,description,properties};
    if(await mutate(()=>invoke<CaseSnapshot>(command,args)))close();
  }
  return <div className={inline?'inline-editor':'overlay'}><section className={inline?'register-editor':'modal'} role={inline?undefined:'dialog'} aria-modal={inline?undefined:true} aria-labelledby="entity-title"><h2 id="entity-title">{request.node?"Edit":"Add"} {label(request.kind)}</h2><form onSubmit={e=>{e.preventDefault();void submit();}}>
    <label>Name<input name="entity-title" autoFocus required disabled={busy} value={title} onChange={e=>setTitle(e.target.value)}/></label>
    <label>Description<textarea name="entity-description" disabled={busy} value={description} onChange={e=>setDescription(e.target.value)}/></label>
    <PropertyFields kind={request.kind} values={properties} set={set} snapshot={snapshot} busy={busy}/>
    <CustomFields kind={request.kind} values={properties} set={set} snapshot={snapshot} busy={busy} mutate={mutate}/>
    {error && <p role="alert" className="error">{error}</p>}<div className="actions"><button type="button" disabled={busy} onClick={close}>Cancel</button><button type="submit" disabled={busy || !title.trim()}>{request.node?"Apply":"Create"}</button></div>
  </form></section></div>;
}
export function LinkEditor({node,snapshot,busy,mutate}:{node:TreeRow;snapshot:CaseSnapshot;busy:boolean;mutate:Props["mutate"]}) {
  const options=relations[node.kind]??[];
  const [relation,setRelation]=useState(options[0]?.relation??"");
  const [target,setTarget]=useState("");
  const activeRelation=options.find(o=>o.relation===relation)??options[0];
  const linked=snapshot.links.filter(l=>l.sourceId===node.id);
  const incoming=snapshot.links.filter(l=>l.targetId===node.id);
  const candidates=snapshot.tree.filter(n=>n.kind===activeRelation?.kind && !linked.some(l=>l.relation===activeRelation.relation && l.targetId===n.id));
  return <section className="links"><h3>Links & references</h3>
    {options.length>0 && <form onSubmit={e=>{e.preventDefault();void mutate(()=>invoke<CaseSnapshot>("case_link",{sourceId:node.id,targetId:target,relation:activeRelation.relation,remove:false})).then(ok=>{if(ok)setTarget("");});}}>
      <label>Relationship<select aria-label="Relationship" value={activeRelation.relation} disabled={busy} onChange={e=>{setRelation(e.target.value);setTarget("");}}>{options.map(o=><option key={o.relation} value={o.relation}>{label(o.relation)}</option>)}</select></label>
      <label>Existing item<select aria-label="Link target" value={target} disabled={busy} onChange={e=>setTarget(e.target.value)}><option value="">Select {label(activeRelation.kind)}</option>{candidates.map(n=><option key={n.id} value={n.id}>{n.title}</option>)}</select></label><button disabled={busy || !target}>Link</button>
    </form>}
    {[...linked,...incoming].map(l=>{const other=snapshot.tree.find(n=>n.id===(l.sourceId===node.id?l.targetId:l.sourceId));return <div className="reference" key={`${l.sourceId}/${l.relation}/${l.targetId}`}><span>{l.sourceId===node.id?"":"Referenced by: "}{label(l.relation)} · {other?.title}</span><button disabled={busy} aria-label={`Unlink ${other?.title}`} onClick={()=>void mutate(()=>invoke<CaseSnapshot>("case_link",{...l,remove:true}))}>Unlink</button></div>;})}
    {!linked.length && !incoming.length && <p className="muted">No assigned references.</p>}
  </section>;
}
