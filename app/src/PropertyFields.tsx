import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import catalog from "./property-catalog.json";
import { label, type CaseSnapshot } from "./model";
export type Field={key:string;label:string;type:string;table?:string;kind?:string;options?:string[];spec:string};
export const fieldsFor=(kind:string):Field[]=>[...catalog.common,...((catalog as unknown as Record<string,Field[]>)[kind]??[])];
export const relations:Record<string,{relation:string;kind:string}[]>={
 hazard:[{relation:"document",kind:"document"},{relation:"accountable",kind:"job_title"},{relation:"sign_off_authority",kind:"job_title"}],
 top_event:[{relation:"document",kind:"document"}],threat:[{relation:"document",kind:"document"}],consequence:[{relation:"document",kind:"document"}],escalation_factor:[{relation:"document",kind:"document"}],product:[{relation:"document",kind:"document"}],
 barrier:[{relation:"activity",kind:"activity"},{relation:"document",kind:"document"},{relation:"system",kind:"system"},{relation:"accountable",kind:"job_title"}],
 activity:[{relation:"document",kind:"document"},{relation:"responsible",kind:"job_title"},{relation:"competency",kind:"competency"},{relation:"input",kind:"product"},{relation:"output",kind:"product"},{relation:"sign_off_authority",kind:"job_title"}],action:[{relation:"action_party",kind:"job_title"},{relation:"sign_off_authority",kind:"job_title"}]
};
export function PropertyFields({kind,values,set,snapshot,busy}:{kind:string;values:Record<string,unknown>;set:(key:string,value:unknown)=>void;snapshot:CaseSnapshot;busy:boolean}) {
 return <div className="property-fields">{fieldsFor(kind).map(f=>{
  const value=values[f.key];
  if(f.type==='checkbox')return <label key={f.key} className="checkbox"><input name={f.key} type="checkbox" checked={value===true} disabled={busy} onChange={e=>set(f.key,e.target.checked)}/>{f.label}</label>;
  const options=f.type==='lookup'?snapshot.lookups.filter(l=>l.tableKey===f.table).map(l=>({id:l.id,title:l.title})):f.type==='entity'?snapshot.tree.filter(n=>n.kind===f.kind).map(n=>({id:n.id,title:n.title})):f.options?.map(v=>({id:v,title:label(v)}));
  return <label key={f.key} title={f.spec}>{f.label}{options?<select name={f.key} disabled={busy} value={String(value??'')} onChange={e=>set(f.key,e.target.value)}><option value="">Unassigned</option>{options.map(o=><option key={o.id} value={o.id}>{o.title}</option>)}</select>:f.type==='textarea'?<textarea name={f.key} value={String(value??'')} disabled={busy} onChange={e=>set(f.key,e.target.value)}/>:<input name={f.key} type={f.type==='integer'?'number':f.type==='date'?'date':'text'} min={f.type==='integer'?0:undefined} step={f.type==='integer'?1:undefined} value={String(value??'')} disabled={busy} onChange={e=>set(f.key,f.type==='integer'?(e.target.value===''?null:Number(e.target.value)):e.target.value)}/>}</label>;
 })}
 {(relations[kind]??[]).map(r=>{
  const links=values.links as Record<string,string[]>|undefined;
  return <label key={r.relation}>{label(r.relation)}<select multiple name={`link.${r.relation}`} value={links?.[r.relation]??[]} disabled={busy} onChange={e=>set('links',{...links,[r.relation]:[...e.target.selectedOptions].map(o=>o.value)})}>{snapshot.tree.filter(n=>n.kind===r.kind).map(n=><option key={n.id} value={n.id}>{n.title}</option>)}</select></label>;
 })}</div>;
}
export function CustomFields({kind,snapshot,values,set,busy,mutate}:{kind:string;snapshot:CaseSnapshot;values:Record<string,unknown>;set:(key:string,value:unknown)=>void;busy:boolean;mutate:(a:()=>Promise<CaseSnapshot>)=>Promise<boolean>}) {
 const [name,setName]=useState(''),[type,setType]=useState('text'),[oldName,setOld]=useState(''),[remove,setRemove]=useState(false);
 const defs=snapshot.customFields[kind]??{},user=(values.user_data??{}) as Record<string,unknown>;
 return <fieldset><legend>User data</legend>{Object.entries(defs).map(([key,t])=><label key={key}>{key}{t==='boolean'?<input type="checkbox" name={`user.${key}`} checked={user[key]===true} disabled={busy} onChange={e=>set('user_data',{...user,[key]:e.target.checked})}/>:<input name={`user.${key}`} type={t==='number'?'number':'text'} step="any" disabled={busy} value={String(user[key]??'')} onChange={e=>set('user_data',{...user,[key]:t==='number'?(e.target.value===''?null:Number(e.target.value)):e.target.value})}/>}</label>)}
 <details><summary>Manage fields for all {label(kind)} items</summary><label>Existing field<select value={oldName} onChange={e=>{setOld(e.target.value);setName(e.target.value);}}><option value="">New field</option>{Object.keys(defs).map(k=><option key={k}>{k}</option>)}</select></label><label>Field name<input name="custom-field-name" value={name} onChange={e=>setName(e.target.value)}/></label><label>Field type<select value={type} disabled={!!oldName} onChange={e=>setType(e.target.value)}><option>text</option><option>number</option><option>boolean</option></select></label><label className="checkbox"><input type="checkbox" checked={remove} onChange={e=>setRemove(e.target.checked)}/>Delete field and its values from all items of this kind</label><button type="button" disabled={busy||!name.trim()||(remove&&!oldName)} onClick={()=>void mutate(()=>invoke<CaseSnapshot>('case_custom_field',{kind,name,fieldType:type,oldName:oldName||null,remove})).then(ok=>{if(ok){const updated={...user};if(oldName){const old=updated[oldName];delete updated[oldName];if(!remove && old!==undefined)updated[name]=old;}set('user_data',updated);setName('');setOld('');setRemove(false);}})}>Apply field definition</button></details></fieldset>;
}
