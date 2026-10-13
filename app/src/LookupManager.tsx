import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import catalog from "./property-catalog.json";
import { label, type CaseSnapshot, type Lookup } from "./model";
import type { Field } from "./PropertyFields";
import { CustomFields } from "./PropertyFields";
import type { Mutate } from "./RiskPanels";
const tables=[...new Set(Object.values(catalog).flat().map(f=>(f as Field).table).filter(Boolean))] as string[];
export default function LookupManager({snapshot,busy,error,mutate}:{snapshot:CaseSnapshot;busy:boolean;error:string;mutate:Mutate}) {
 const [table,setTable]=useState('effectiveness'),[edit,setEdit]=useState<Lookup|null>(null);
 return <section><h2>Property choices & effectiveness</h2><label>Taxonomy<select aria-label="Taxonomy" value={table} disabled={busy} onChange={e=>setTable(e.target.value)}>{tables.map(t=><option key={t} value={t}>{label(t)}</option>)}</select></label><button disabled={busy} onClick={()=>setEdit({id:crypto.randomUUID(),tableKey:table,title:'',code:'',color:'#d9e9d9',sortOrder:snapshot.lookups.filter(l=>l.tableKey===table).length,metadata:{description:'',frequency:null}})}>New choice</button>
 {snapshot.lookups.filter(l=>l.tableKey===table).map(l=><div className="matrix-summary" key={l.id}><span style={{background:l.color,width:20,height:20}}/><strong>{l.title}</strong><span>{l.code} · order {l.sortOrder}</span><button disabled={busy} onClick={()=>setEdit(l)}>Edit choice</button><button disabled={busy} onClick={()=>void mutate(()=>invoke<CaseSnapshot>('case_delete_lookup',{id:l.id}))}>Delete choice</button></div>)}
 {edit&&<div className="overlay"><section className="modal" role="dialog" aria-modal="true"><h2>{label(edit.tableKey)} choice</h2><label>Name<input name="choice-name" value={edit.title} onChange={e=>setEdit({...edit,title:e.target.value})}/></label><label>Code<input name="choice-code" value={edit.code} onChange={e=>setEdit({...edit,code:e.target.value})}/></label><label>Color<input name="choice-color" type="color" value={edit.color} onChange={e=>setEdit({...edit,color:e.target.value})}/></label><label>Order (low to high)<input name="choice-order" type="number" min="0" step="1" value={edit.sortOrder} onChange={e=>setEdit({...edit,sortOrder:Number(e.target.value)})}/></label><label>Description<textarea name="choice-description" value={String(edit.metadata.description??'')} onChange={e=>setEdit({...edit,metadata:{...edit.metadata,description:e.target.value}})}/></label>{table==='effectiveness'&&<label>Optional frequency metadata<input name="effectiveness-frequency" type="number" min="0" step="any" value={String(edit.metadata.frequency??'')} onChange={e=>setEdit({...edit,metadata:{...edit.metadata,frequency:e.target.value===''?null:Number(e.target.value)}})}/></label>}
 <CustomFields kind={`lookup:${edit.tableKey}`} snapshot={snapshot} values={edit.metadata} set={(key,value)=>setEdit({...edit,metadata:{...edit.metadata,[key]:value}})} busy={busy} mutate={mutate}/>
 {error&&<p role="alert" className="error">{error}</p>}<div className="actions"><button disabled={busy} onClick={()=>setEdit(null)}>Cancel</button><button disabled={busy||!edit.title.trim()} onClick={()=>void mutate(()=>invoke<CaseSnapshot>('case_save_lookup',{lookup:edit})).then(ok=>{if(ok)setEdit(null);})}>Save choice</button></div></section></div>}
 </section>;
}
