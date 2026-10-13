import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

type Difference = { table: string; key: string; field: string; status: "added" | "removed" | "changed"; before: unknown; after: unknown };
type Comparison = { leftTitle: string; rightTitle: string; signature: string; differences: Difference[] };
export default function CaseCompare({ busy, run }: { busy: boolean; run: (action: () => Promise<void>) => Promise<boolean> }) {
  const [path, setPath] = useState("");
  const [reportPath, setReportPath] = useState("");
  const [result, setResult] = useState<Comparison | null>(null);
  const [filter, setFilter] = useState("all");
  const [saved, setSaved] = useState("");
  const shown = result?.differences.filter(d => filter === "all" || d.status === filter) ?? [];
  return <section className="case-compare"><h2>Compare cases</h2>
    <p>Compare the current case, including unsaved edits, with a validated file opened read-only. Differences read from the current case to the second case. Records are matched by their stored identities; independently created records appear as additions and removals.</p>
    <label>Second case<input name="comparison-path" value={path} disabled={busy} onChange={e => { setPath(e.target.value); setResult(null); setSaved(""); }} /></label>
    <div className="actions"><button disabled={busy} onClick={() => void run(async () => { const chosen = await open({ title: "Choose comparison case", multiple: false, filters: [{ name: "Case files", extensions: ["sqlite"] }] }); if (typeof chosen === "string") { setPath(chosen); setResult(null); setSaved(""); } })}>Browse second case</button>
      <button disabled={busy || !path.trim()} onClick={() => void run(async () => { setResult(null); setSaved(""); setResult(await invoke<Comparison>("case_compare", { path })); })}>Compare</button></div>
    {result && <><h3>{result.leftTitle} → {result.rightTitle}</h3><p role="status">{result.differences.length} differences</p>
      <label>Show<select name="comparison-filter" value={filter} disabled={busy} onChange={e => setFilter(e.target.value)}>{["all", "added", "removed", "changed"].map(s => <option key={s} value={s}>{s}</option>)}</select></label>
      <div className="table-scroll"><table className="diff-table"><thead><tr><th>Status</th><th>Record</th><th>Field</th><th>Current case</th><th>Second case</th></tr></thead><tbody>{shown.map((d, i) => <tr key={i} className={`diff-${d.status}`}><td>{d.status}</td><td>{d.table}<small>{d.key}</small></td><td>{d.field}</td><td><pre>{JSON.stringify(d.before, null, 2)}</pre></td><td><pre>{JSON.stringify(d.after, null, 2)}</pre></td></tr>)}</tbody></table></div>
      <label>Diff report destination<input name="comparison-report-path" disabled={busy} value={reportPath} onChange={e => { setReportPath(e.target.value); setSaved(""); }} /></label><div className="actions">
        <button disabled={busy} onClick={() => void run(async () => { const chosen = await save({ title: "Save diff report", defaultPath: "case-differences.txt", filters: [{ name: "Text report", extensions: ["txt"] }] }); if (chosen) { setReportPath(chosen); setSaved(""); } })}>Browse report destination</button>
        <button disabled={busy || !reportPath.trim()} onClick={() => void run(async () => { setSaved(""); await invoke("case_comparison_report", { path, destination: reportPath, signature: result.signature }); setSaved(reportPath); })}>Save diff report</button></div>
      {saved && <p role="status">Diff report saved: {saved}</p>}</>}
  </section>;
}
