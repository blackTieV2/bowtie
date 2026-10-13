import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { CaseSnapshot } from "./model";
import type { Mutate } from "./RiskPanels";
type Change = { entityId: string; path: string; field: string; before: string; after: string; occurrences: number };
type Preview = { signature: string; changes: Change[] };
type Applied = { snapshot: CaseSnapshot; changedFields: number; occurrences: number };
export default function FindReplace({ busy, run, mutate }: { busy: boolean; run: (a: () => Promise<void>) => Promise<boolean>; mutate: Mutate }) {
  const [find, setFind] = useState(""), [replacement, setReplacement] = useState("");
  const [preview, setPreview] = useState<Preview | null>(null), [selected, setSelected] = useState<number[]>([]), [result, setResult] = useState("");
  function reset() { setPreview(null); setSelected([]); setResult(""); }
  async function apply(indices: number[]) {
    if (!preview) return;
    let applied: Applied | undefined;
    const ok = await mutate(async () => { applied = await invoke<Applied>("case_replace_apply", { find, replacement, signature: preview.signature, selected: indices }); return applied.snapshot; });
    if (ok && applied) { setPreview(null); setSelected([]); setResult(`Replaced ${applied.occurrences} occurrences across ${applied.changedFields} fields.`); }
  }
  return <section><h2>Find & replace</h2><p>Literal, case-sensitive text across the case: titles, descriptions, editable text properties and text user fields, including lookup and matrix labels. Review each full value and entity path before applying. Each selected field replaces all its matching occurrences; titles are trimmed as in the editor.</p>
    <div className="property-fields"><label>Find<input name="replace-find" value={find} disabled={busy} onChange={e => { setFind(e.target.value); reset(); }} /></label><label>Replace with<input name="replace-with" value={replacement} disabled={busy} onChange={e => { setReplacement(e.target.value); reset(); }} /></label></div>
    <button disabled={busy || !find} onClick={() => void run(async () => { reset(); const next = await invoke<Preview>("case_replace_preview", { find, replacement }); setPreview(next); setSelected(next.changes.map((_, i) => i)); })}>Preview replacements</button>
    {preview && <><p role="status">{preview.changes.length} pending field changes</p><div className="table-scroll"><table className="diff-table"><thead><tr><th>Select</th><th>Entity path</th><th>Field</th><th>Before</th><th>After</th><th>Occurrences</th><th>Apply</th></tr></thead><tbody>{preview.changes.map((c, i) => <tr key={i}><td><input type="checkbox" aria-label={`Select change ${i + 1}`} checked={selected.includes(i)} disabled={busy} onChange={e => setSelected(e.target.checked ? [...selected, i] : selected.filter(n => n !== i))} /></td><td>{c.path}</td><td>{c.field}</td><td><pre>{c.before}</pre></td><td><pre>{c.after}</pre></td><td>{c.occurrences}</td><td><button disabled={busy} onClick={() => void apply([i])}>Replace this field</button></td></tr>)}</tbody></table></div>
      <div className="actions"><button disabled={busy || !selected.length} onClick={() => void apply(selected)}>Apply selected replacements</button><button disabled={busy || !preview.changes.length} onClick={() => void apply(preview.changes.map((_, i) => i))}>Replace all previewed fields</button></div></>}
    {result && <p role="status">{result}</p>}<div className="actions"><button disabled={busy} onClick={() => void mutate(() => invoke<CaseSnapshot>("case_tool_undo")).then(ok => { if (ok) { reset(); setResult("Last tool operation undone."); } })}>Undo last tool operation</button></div>
  </section>;
}
