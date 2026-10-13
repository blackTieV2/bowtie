import { useEffect, useState, type ReactNode } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";
import { diagramFor, label, type CaseSnapshot, type TreeRow } from "./model";
import DiagramCanvas from "./DiagramCanvas";
import EntityEditor, { LinkEditor, type EditRequest } from "./EntityEditor";
import { MatrixManager, RiskPanel } from "./RiskPanels";
import LookupManager from "./LookupManager";
import ActionRegister from "./ActionRegister";
import { fieldsFor } from "./PropertyFields";
import ExportDialog from "./ExportDialog";
import CaseCompare from "./CaseCompare";
import FindReplace from "./FindReplace";
import CaseExchange from "./CaseExchange";
import AuditPanel,{AuditTargetResults} from "./AuditPanel";

type FormKind = "new" | "group" | "diagram";
type Navigation = "new" | "open" | "close" | "quit";
const desktop = isTauri();

export default function App() {
  const [active, setActive] = useState<CaseSnapshot | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [form, setForm] = useState<FormKind | null>(null);
  const [title, setTitle] = useState("");
  const [organization, setOrganization] = useState("");
  const [hazard, setHazard] = useState("");
  const [topEvent, setTopEvent] = useState("");
  const [pending, setPending] = useState<Navigation | null>(null);
  const [editRequest,setEditRequest] = useState<EditRequest | null>(null);
  const [deleteRequest,setDeleteRequest] = useState<TreeRow | null>(null);
  const [lastDiagram,setLastDiagram] = useState<string | null>(null);
  const [view,setView] = useState('diagram');
  const [exportOpen,setExportOpen] = useState(false);
  const current = active?.tree.find(row => row.id === selected);
  const diagram = active ? diagramFor(active.tree,current) ?? active.tree.find(n=>n.id===lastDiagram && n.kind==='diagram') : undefined;
  useEffect(()=>{if(diagram)setLastDiagram(diagram.id);},[diagram?.id]);

  function accept(snapshot: CaseSnapshot) {
    setActive(snapshot);
    setSelected(previous => snapshot.tree.some(row => row.id === previous) ? previous : snapshot.tree[0]?.id ?? null);
  }
  async function operation(action: () => Promise<void>): Promise<boolean> {
    setBusy(true); setError("");
    try { await action(); return true; }
    catch (reason) { setError(String(reason)); return false; }
    finally { setBusy(false); }
  }
  async function mutate(action:()=>Promise<CaseSnapshot>):Promise<boolean> {
    return operation(async()=>{
      const snapshot=await action();
      const newEntity=snapshot.tree.find(row=>!active?.tree.some(old=>old.id===row.id) && row.kind!=="hazard" && row.kind!=="diagram");
      accept(snapshot);if(newEntity)setSelected(newEntity.id);
    });
  }
  function addEntity(kind:string,parentId=current?.id) { if(parentId){setError("");setEditRequest({kind,parentId});} }
  function editEntity(node:TreeRow) {setError("");setEditRequest({node,parentId:node.parentId??"",kind:node.kind});}
  useEffect(() => {
    if (!desktop) return;
    invoke<CaseSnapshot | null>("case_get").then(snapshot => { if (snapshot) accept(snapshot); }).catch(reason => setError(String(reason)));
  }, []);
  useEffect(() => {
    if (!desktop) return;
    let disposed = false;
    let unlisten: (() => void) | undefined;
    getCurrentWindow().onCloseRequested(event => {
      if (busy || active?.dirty) { event.preventDefault(); if (!busy) setPending("quit"); }
    }).then(stop => { if (disposed) stop(); else unlisten = stop; }).catch(reason => setError(String(reason)));
    return () => { disposed = true; unlisten?.(); };
  }, [active?.dirty, busy]);

  async function save(asCopy = false): Promise<boolean> {
    if (!active) return false;
    let path = asCopy ? null : active.path;
    if (!path) {
      const safeName = active.title.replace(/[<>:"/\\|?*\x00-\x1f]/g, "_");
      try { path = await saveDialog({ title: "Save case", defaultPath: active.path ?? `${safeName}.sqlite`, filters: [{ name: "Case files", extensions: ["sqlite"] }] }); }
      catch (reason) { setError(String(reason)); return false; }
      if (!path) return false;
    }
    return operation(async () => accept(await invoke<CaseSnapshot>("case_save", { path })));
  }
  async function navigate(action: Navigation) {
    if (action === "new") { setTitle(""); setOrganization(""); setForm("new"); return; }
    if (action === "open") {
      await operation(async () => {
        const path = await openDialog({ title: "Open case", multiple: false, directory: false, filters: [{ name: "Case files", extensions: ["sqlite"] }] });
        if (typeof path === "string") accept(await invoke<CaseSnapshot>("case_open", { path }));
      });
    } else if (action === "close") {
      await operation(async () => { await invoke("case_close"); setActive(null); setSelected(null); });
    } else await operation(async () => { await getCurrentWindow().destroy(); });
  }
  function request(action: Navigation) { if (active?.dirty) setPending(action); else void navigate(action); }
  async function resolvePending(choice: "save" | "discard") {
    const action = pending;
    if (!action || (choice === "save" && !(await save()))) return;
    setPending(null);
    await navigate(action);
  }
  async function submit() {
    const ok = await operation(async () => {
      let snapshot:CaseSnapshot | undefined;
      if (form === "new") snapshot=await invoke<CaseSnapshot>("case_create", { title, organization });
      else if (form === "group" && current) snapshot=await invoke<CaseSnapshot>("case_add_group", { parentId: current.id, title });
      else if (form === "diagram" && current) snapshot=await invoke<CaseSnapshot>("case_add_diagram", { parentId: current.id, hazardTitle: hazard, topEventTitle: topEvent });
      if(snapshot){const added=snapshot.tree.filter(row=>!active?.tree.some(old=>old.id===row.id));accept(snapshot);setSelected(added.at(-1)?.id??snapshot.tree[0]?.id??null);}
    });
    if (ok) setForm(null);
  }
  function tree(parent: string | null): ReactNode {
    return active?.tree.filter(row => row.parentId === parent).map(row => {
      const children = active.tree.some(child => child.parentId === row.id);
      const item = <button data-entity-id={row.id} className={`tree-item ${selected === row.id ? "selected" : ""}`} onClick={() => setSelected(row.id)} onDoubleClick={()=>editEntity(row)} disabled={busy} aria-pressed={selected === row.id}><span>{row.title}</span><small>{label(row.kind)}</small></button>;
      return <li key={row.id}>{children ? <details open><summary aria-label={`Expand ${row.title}`}><span aria-hidden="true">▸</span></summary>{item}<ul>{tree(row.id)}</ul></details> : item}</li>;
    });
  }
  const formValid = form === "new" ? title.trim() && organization.trim() : form === "group" ? title.trim() : topEvent.trim() && (current?.kind === "hazard" || hazard.trim());
  return <div className="workspace">
    <header><span className="mark" aria-hidden="true">◇</span><strong>Barrier Atlas</strong><span className="local">Local workspace</span></header>
    <nav aria-label="Case controls">
      <button disabled={!desktop || busy} onClick={() => request("new")}>New case</button><button disabled={!desktop || busy} onClick={() => request("open")}>Open</button>
      <button disabled={!active || busy} onClick={() => void save()}>Save</button><button disabled={!active || busy} onClick={() => void save(true)}>Save as</button>
      <button disabled={!active || busy} onClick={() => request("close")}>Close case</button><span className="case-title">{active ? `${active.title}${active.dirty ? " *" : ""}` : "No case open"}</span>
      <button disabled={!active||busy} onClick={()=>{setError('');setExportOpen(true);}}>Export</button>
    </nav>
    <aside aria-label="Case explorer"><h2>Case explorer</h2>{active ? <ul className="tree">{tree(null)}</ul> : <p className="muted">Create or open a local case.</p>}</aside>
    <main>
      {error && <p className="error" role="alert">{error}</p>}{!desktop && <p className="notice">Use the desktop app to create and open local case files.</p>}
      {active&&<div className="workspace-tabs" aria-label="Workspace views">{[['diagram','Diagram'],['matrices','Risk matrices'],['choices','Property choices'],['actions','Action register'],['compare','Compare cases'],['replace','Find & replace'],['exchange','CSV & spreadsheet'],['audits','Audits']].map(([key,title])=><button key={key} disabled={busy} aria-pressed={view===key} onClick={()=>setView(key)}>{title}</button>)}</div>}
      {active&&view==='audits'&&<AuditPanel key={active.path??active.tree[0]?.id} snapshot={active} selected={selected} busy={busy} error={error} mutate={mutate} run={operation}/>}
      {active&&view==='compare'&&<CaseCompare key={active.path??active.tree[0]?.id} busy={busy} run={operation}/>}
      {active&&view==='replace'&&<FindReplace key={active.path??active.tree[0]?.id} busy={busy} run={operation} mutate={mutate}/>}
      {active&&view==='exchange'&&<CaseExchange key={active.path??active.tree[0]?.id} busy={busy} run={operation} mutate={mutate}/>}
      {active&&view==='matrices'&&<MatrixManager snapshot={active} busy={busy} error={error} mutate={mutate}/>}
      {active&&view==='choices'&&<LookupManager snapshot={active} busy={busy} error={error} mutate={mutate}/>}
      {active&&view==='actions'&&<ActionRegister snapshot={active} busy={busy} error={error} mutate={mutate}/>}
      {active && view==='diagram' ? <><section className="case-summary"><p className="eyebrow">{label(current?.kind ?? "case")}</p><h1>{current?.title ?? active.title}</h1>
        <div className="actions"><button disabled={busy || !current || !["organization", "group"].includes(current.kind)} onClick={() => { setTitle(""); setForm("group"); }}>Add group</button>
        <button disabled={busy || !current || !["group", "hazard"].includes(current.kind)} onClick={() => { setHazard(current?.kind === "hazard" ? current.title : ""); setTopEvent(""); setForm("diagram"); }}>Add diagram</button></div>
        {current && <div className="actions entity-tools" aria-label="Diagram editing controls">
          {current.kind==='top_event' && <><button disabled={busy} onClick={()=>addEntity('threat')}>Add threat</button><button disabled={busy} onClick={()=>addEntity('consequence')}>Add consequence</button></>}
          {['threat','consequence','escalation_factor'].includes(current.kind) && <button disabled={busy} onClick={()=>addEntity('barrier')}>Add barrier</button>}
          {current.kind==='barrier' && <button disabled={busy} onClick={()=>addEntity('escalation_factor')}>Add escalation factor</button>}
          {['hazard','top_event','threat','consequence','barrier','escalation_factor','activity'].includes(current.kind) && <button disabled={busy} onClick={()=>addEntity('action')}>Add action</button>}
          <button disabled={busy} onClick={()=>editEntity(current)}>Edit</button><button disabled={busy} onClick={()=>{setError('');setDeleteRequest(current);}}>Delete</button>
          {current.kind==='top_event'&&<button disabled={busy} onClick={()=>{const d=active.tree.find(n=>n.id===current.parentId);const h=active.tree.find(n=>n.id===d?.parentId);if(h)editEntity(h);}}>Edit hazard details</button>}
        </div>}
      </section>
      {diagram && <DiagramCanvas snapshot={active} diagram={diagram} selected={selected} busy={busy} mutate={mutate} select={setSelected} edit={editEntity} persist={positions=>mutate(()=>invoke<CaseSnapshot>('case_set_positions',{diagramId:diagram.id,positions}))}/>}
      <section className="support-panel"><div className="support-tools"><h3>Supporting items</h3>{['activity','document','job_title','product','system','competency'].map(kind=><button key={kind} disabled={busy} onClick={()=>addEntity(kind,kind===current?.kind && ['activity','document','system'].includes(kind)?current.id:active.tree.find(n=>n.kind==='organization')?.id)}>New {label(kind)}</button>)}</div>
      {current && <><p className="muted">{current.description}</p>{Object.entries(current.properties).filter(([k,v])=>!['canvas.position','custom_definitions','user_data'].includes(k)&&v!==''&&v!==null).map(([k,v])=><p key={k}><strong>{fieldsFor(current.kind).find(f=>f.key===k)?.label??label(k)}:</strong> {active.lookups.find(l=>l.id===v)?.title??active.tree.find(n=>n.id===v)?.title??String(v)}</p>)}<RiskPanel key={`risk/${current.id}`} node={current} snapshot={active} busy={busy} mutate={mutate}/><LinkEditor key={current.id} node={current} snapshot={active} busy={busy} mutate={mutate}/></>}</section>
      {current&&['barrier','activity','document'].includes(current.kind)&&<AuditTargetResults key={`audit/${current.id}`} snapshot={active} targetId={current.id}/>}
      </> : !active&&<section className="empty"><h1>A place to understand risk</h1><p>Hazards, barriers, and the work that keeps them effective.</p></section>}
    </main>
    <footer>{busy ? "Working…" : "Offline · No account required"}</footer>
    {exportOpen&&active&&<ExportDialog diagram={diagram} busy={busy} error={error} run={operation} close={()=>{setExportOpen(false);setError('');}}/>}
    {editRequest && active && <EntityEditor key={editRequest.node?.id??editRequest.kind} snapshot={active} request={editRequest} busy={busy} error={error} close={()=>{setEditRequest(null);setError('');}} mutate={mutate}/>}
    {deleteRequest && <div className="overlay"><section className="modal" role="dialog" aria-modal="true" aria-labelledby="delete-title"><h2 id="delete-title">Delete {deleteRequest.title}?</h2><p>Children and assigned references must be removed first. Shared items are preserved.</p>{error && <p className="error" role="alert">{error}</p>}<div className="actions"><button disabled={busy} onClick={()=>{setDeleteRequest(null);setError('');}}>Cancel</button><button disabled={busy} onClick={()=>void mutate(()=>invoke<CaseSnapshot>('case_delete_entity',{id:deleteRequest.id})).then(ok=>{if(ok)setDeleteRequest(null);})}>Delete item</button></div></section></div>}
    {form && <div className="overlay"><section role="dialog" aria-modal="true" aria-labelledby="form-title" className="modal"><h2 id="form-title">{form === "new" ? "New case" : form === "group" ? "New group" : "New diagram"}</h2>
      <form onSubmit={event => { event.preventDefault(); void submit(); }}>
        {form === "new" || form === "group" ? <label>{form === "new" ? "Case name" : "Group name"}<input autoFocus required value={title} onChange={event => setTitle(event.target.value)} disabled={busy} /></label> : <label>Hazard<input autoFocus required value={hazard} onChange={event => setHazard(event.target.value)} disabled={busy || current?.kind === "hazard"} /></label>}
        {form === "new" && <label>Organization<input required value={organization} onChange={event => setOrganization(event.target.value)} disabled={busy} /></label>}
        {form === "diagram" && <label>Top event<input required value={topEvent} onChange={event => setTopEvent(event.target.value)} disabled={busy} /></label>}
        {error && <p className="error" role="alert">{error}</p>}<div className="actions"><button type="button" disabled={busy} onClick={() => { setForm(null); setError(""); }}>Cancel</button><button type="submit" disabled={busy || !formValid}>Create</button></div>
      </form></section></div>}
    {pending && <div className="overlay"><section role="dialog" aria-modal="true" aria-labelledby="unsaved-title" className="modal"><h2 id="unsaved-title">Save your changes?</h2><p>This case has changes that have not been saved.</p>{error && <p className="error" role="alert">{error}</p>}<div className="actions"><button disabled={busy} onClick={() => { setPending(null); setError(""); }}>Cancel</button><button disabled={busy} onClick={() => void resolvePending("discard")}>Discard</button><button disabled={busy} onClick={() => void resolvePending("save")}>Save</button></div></section></div>}
  </div>;
}
