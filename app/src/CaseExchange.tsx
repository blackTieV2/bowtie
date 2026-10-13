import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { CaseSnapshot } from "./model";
import type { Mutate } from "./RiskPanels";
import { fromWorkbook, toWorkbook } from "./tableWorkbook";
type Preview = { signature: string; counts: Record<string, number>; errors: { line: number; message: string }[]; updatesEmptyCaseHeader: boolean };
export default function CaseExchange({ busy, run, mutate }: { busy: boolean; run: (a: () => Promise<void>) => Promise<boolean>; mutate: Mutate }) {
  const [path, setPath] = useState(""), [text, setText] = useState(""), [preview, setPreview] = useState<Preview | null>(null), [status, setStatus] = useState("");
  const [sheets, setSheets] = useState<{ name: string; text: string }[]>([]), [sheet, setSheet] = useState(0);
  function reset() { setPreview(null); setStatus(""); }
  return <section><h2>CSV & spreadsheet exchange</h2><p>The original eight-column table contains entities (including actions), links, lookups, matrices and assessments. Export then edit values in CSV or XLSX. Import creates records, rejects duplicate identities and validates the entire batch before apply. A new empty case adopts the package header; a populated case requires matching organization details.</p>
    <label>Input or output file<input name="table-path" value={path} disabled={busy} onChange={e => { setPath(e.target.value); reset(); }} /></label>
    <div className="actions"><button disabled={busy} onClick={() => void run(async () => { const chosen = await open({ title: "Read case table", multiple: false, filters: [{ name: "Case tables", extensions: ["csv", "xlsx"] }] }); if (typeof chosen === "string") setPath(chosen); })}>Browse input</button>
      <button disabled={busy || !path} onClick={() => void run(async () => { reset(); const bytes = new Uint8Array(await invoke<number[]>("case_read_table", { path })); if (path.toLowerCase().endsWith(".xlsx")) { const loaded = fromWorkbook(bytes); if (!loaded.length) throw Error("No worksheets found"); setSheets(loaded); setSheet(0); setText(loaded[0].text); } else { setSheets([]); setText(new TextDecoder("utf-8", { fatal: true }).decode(bytes)); } })}>Read table</button>
      <button disabled={busy} onClick={() => void run(async () => { const chosen = await save({ title: "Export case table", defaultPath: "case-data.csv", filters: [{ name: "CSV table", extensions: ["csv"] }, { name: "Spreadsheet", extensions: ["xlsx"] }] }); if (chosen) setPath(chosen); })}>Browse output</button>
      <button disabled={busy || !path} onClick={() => void run(async () => { reset(); const exported = await invoke<{ text: string; signature: string }>("case_table_export"); const bytes = path.toLowerCase().endsWith(".xlsx") ? toWorkbook(exported.text) : new TextEncoder().encode(exported.text); await invoke("case_write_table", { path, bytes: Array.from(bytes), signature: exported.signature }); setStatus(`Table exported: ${path}`); })}>Export case table</button></div>
    {sheets.length > 0 && <label>Worksheet<select name="table-sheet" value={sheet} disabled={busy} onChange={e => { const i = Number(e.target.value); setSheet(i); setText(sheets[i].text); reset(); }}>{sheets.map((s, i) => <option key={i} value={i}>{s.name}</option>)}</select></label>}
    <label>Import table content<textarea name="table-content" value={text} disabled={busy} onChange={e => { setText(e.target.value); setSheets([]); reset(); }} /></label>
    <button disabled={busy || !text} onClick={() => void run(async () => { reset(); setPreview(await invoke<Preview>("case_import_preview", { text })); })}>Validate & preview import</button>
    {preview && <section aria-label="Import dry run"><h3>Dry-run summary</h3><p>{Object.entries(preview.counts).map(([type, count]) => `${type}: ${count}`).join(" · ") || "No new records"}</p>{preview.updatesEmptyCaseHeader && <p>The empty case header and organization details will adopt the package values.</p>}
      {preview.errors.length ? <><p role="alert">Import rejected. The case is unchanged.</p><ul>{preview.errors.map((e, i) => <li key={i}>Line {e.line}: {e.message}</li>)}</ul></> : <><p role="status">Validation passed. Ready to create the listed records in one transaction.</p><button disabled={busy} onClick={() => { let counts: Record<string, number> | undefined; void mutate(async () => { const applied = await invoke<{ snapshot: CaseSnapshot; counts: Record<string, number> }>("case_import_apply", { text, signature: preview.signature }); counts = applied.counts; return applied.snapshot; }).then(ok => { if (ok) { setPreview(null); setStatus(`Import applied: ${Object.entries(counts ?? {}).map(([type, count]) => `${type}: ${count}`).join(" · ")}`); } }); }}>Apply validated import</button></>}
    </section>}{status && <p role="status">{status}</p>}
    <button disabled={busy} onClick={() => void mutate(() => invoke<CaseSnapshot>("case_tool_undo")).then(ok => { if (ok) { reset(); setStatus("Last tool operation undone."); } })}>Undo last tool operation</button>
  </section>;
}
