import { useEffect, useState, type ReactNode } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";

type TreeRow = { id: string; parentId: string | null; kind: string; title: string; sortOrder: number; depth: number };
type CaseSnapshot = { title: string; path: string | null; dirty: boolean; tree: TreeRow[] };
type FormKind = "new" | "group" | "diagram";
type Navigation = "new" | "open" | "close" | "quit";
const desktop = isTauri();
const label = (kind: string) => kind.replaceAll("_", " ");

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
  const current = active?.tree.find(row => row.id === selected);

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
      if (form === "new") accept(await invoke<CaseSnapshot>("case_create", { title, organization }));
      else if (form === "group" && current) accept(await invoke<CaseSnapshot>("case_add_group", { parentId: current.id, title }));
      else if (form === "diagram" && current) accept(await invoke<CaseSnapshot>("case_add_diagram", { parentId: current.id, hazardTitle: hazard, topEventTitle: topEvent }));
    });
    if (ok) setForm(null);
  }
  function tree(parent: string | null): ReactNode {
    return active?.tree.filter(row => row.parentId === parent).map(row => {
      const children = active.tree.some(child => child.parentId === row.id);
      const item = <button className={`tree-item ${selected === row.id ? "selected" : ""}`} onClick={() => setSelected(row.id)} disabled={busy} aria-pressed={selected === row.id}><span>{row.title}</span><small>{label(row.kind)}</small></button>;
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
    </nav>
    <aside aria-label="Case explorer"><h2>Case explorer</h2>{active ? <ul className="tree">{tree(null)}</ul> : <p className="muted">Create or open a local case.</p>}</aside>
    <main>
      {error && <p className="error" role="alert">{error}</p>}{!desktop && <p className="notice">Use the desktop app to create and open local case files.</p>}
      {active ? <section className="case-summary"><p className="eyebrow">{label(current?.kind ?? "case")}</p><h1>{current?.title ?? active.title}</h1><p className="muted">{active.path ?? "Not saved yet"}</p>
        <div className="actions"><button disabled={busy || !current || !["organization", "group"].includes(current.kind)} onClick={() => { setTitle(""); setForm("group"); }}>Add group</button>
        <button disabled={busy || !current || !["group", "hazard"].includes(current.kind)} onClick={() => { setHazard(current?.kind === "hazard" ? current.title : ""); setTopEvent(""); setForm("diagram"); }}>Add diagram</button></div>
        <p className="muted">{active.tree.length} items in this case</p>
      </section> : <section className="empty"><h1>A place to understand risk</h1><p>Hazards, barriers, and the work that keeps them effective.</p></section>}
    </main>
    <footer>{busy ? "Working…" : "Offline · No account required"}</footer>
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
