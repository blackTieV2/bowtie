// Diagnostics of the repository's own built app, using an isolated WebView profile.
import assert from 'node:assert/strict';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { createHash } from 'node:crypto';
const sleep = ms => new Promise(r => setTimeout(r, ms));
const port = Number(process.env.BARRIER_ATLAS_CDP_PORT ?? 9336);
let target;
for (let i = 0; i < 120; i++) {
  try { target = (await (await fetch(`http://127.0.0.1:${port}/json/list`)).json()).find(t => t.type === 'page' && t.webSocketDebuggerUrl); if (target) break; } catch {}
  await sleep(250);
}
assert.ok(target, 'Own app diagnostic target did not appear');
const socket = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((done, fail) => { socket.addEventListener('open', done, { once: true }); socket.addEventListener('error', fail, { once: true }); });
let next = 0;
const pending = new Map();
socket.addEventListener('message', e => { const m = JSON.parse(String(e.data)), r = pending.get(m.id); if (!r) return; pending.delete(m.id); clearTimeout(r.timer); m.error ? r.reject(Error(m.error.message)) : r.resolve(m.result); });
function send(method, params = {}) { return new Promise((resolve, reject) => { const id = ++next, timer = setTimeout(() => { pending.delete(id); reject(Error(`Timed out ${method}`)); }, 30000); pending.set(id, { resolve, reject, timer }); socket.send(JSON.stringify({ id, method, params })); }); }
async function evaluate(expression) { const r = await send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true }); assert.ok(!r.exceptionDetails, JSON.stringify(r.exceptionDetails)); return r.result.value; }
async function waitFor(expression) { for (let i = 0; i < 150; i++) { if (await evaluate(expression)) return; await sleep(100); } throw Error(`Condition failed: ${expression}`); }
const invoke = (command, args = {}) => evaluate(`window.__TAURI_INTERNALS__.invoke(${JSON.stringify(command)},${JSON.stringify(args)})`);
async function button(text) { assert.ok(await evaluate(`(()=>{const b=[...document.querySelectorAll('button')].find(b=>b.textContent.trim()===${JSON.stringify(text)});if(!b||b.disabled)return false;b.click();return true;})()`), text); await sleep(80); }
async function field(selector, value) { assert.ok(await evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e)return false;Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set.call(e,${JSON.stringify(value)});e.dispatchEvent(new Event('input',{bubbles:true}));return true;})()`), selector); await sleep(60); }
const runRoot=resolve(process.env.BARRIER_ATLAS_PHASE5_RUN??'runs/006-build-phase5');
const folder = resolve(process.env.BARRIER_ATLAS_SMOKE_OUTPUT??'.runtime/phase5-comparison','comparison'), evidence = resolve(runRoot,'evidence');
try {
  await mkdir(folder, { recursive: true });
  await mkdir(evidence, { recursive: true });
  await waitFor("!!window.__TAURI_INTERNALS__ && document.body.innerText.includes('Barrier Atlas')");
  const start = await invoke('case_create', { title: 'Comparison review', organization: 'Workshop' });
  const root = start.tree[0].id;
  const second = resolve(folder, 'second.sqlite'), report = resolve(folder, 'differences.txt');
  await invoke('case_save', { path: second });
  const secondBytes = await readFile(second);
  await invoke('case_add_group', { parentId: root, title: 'Unsaved group' });
  const before = await invoke('case_get');
  await send('Page.reload');
  await waitFor("document.body.innerText.includes('Comparison review')");
  await button('Compare cases');
  await field('[name="comparison-path"]', second);
  await button('Compare');
  await waitFor("document.body.innerText.includes('1 differences')");
  assert.ok(await evaluate("document.querySelector('.diff-table tbody tr').classList.contains('diff-removed')"));
  await field('[name="comparison-report-path"]', report);
  await button('Save diff report');
  await waitFor("document.body.innerText.includes('Diff report saved:')");
  const reportBytes = await readFile(report);
  assert.equal(JSON.parse(reportBytes).differences[0].status, 'removed');
  assert.deepEqual(await invoke('case_get'), before);
  assert.deepEqual(await readFile(second), secondBytes);
  const capture = await send('Page.captureScreenshot', { format: 'png' });
  await writeFile(resolve(evidence, 'comparison-workspace.png'), Buffer.from(capture.data, 'base64'));
  // Both stale active data and malformed comparison sources must surface errors.
  await invoke('case_add_group', { parentId: root, title: 'Later edit' });
  await button('Save diff report');
  await waitFor("document.body.innerText.includes('changed since the comparison')");
  assert.deepEqual(await readFile(report), reportBytes);
  const invalid = resolve(folder, 'invalid.sqlite');
  await writeFile(invalid, 'invalid');
  await field('[name="comparison-path"]', invalid);
  await button('Compare');
  await waitFor("!!document.querySelector('[role=alert]')");
  assert.equal(await evaluate("document.querySelectorAll('.diff-table').length"), 0);
  assert.deepEqual(await readFile(second), secondBytes);
  assert.equal(await readFile(invalid, 'utf8'), 'invalid');
  const result = { result: 'PASS', scope: 'Windows native comparison checks; full Phase 5 checked separately', guiPreview: true, guiTextReport: true, unsavedCurrentCaseIncluded: true, activeSessionUnchangedByComparison: true, secondFileByteEquality: true, staleReportRejected: true, invalidSecondCaseRejected: true, reportSha256: createHash('sha256').update(reportBytes).digest('hex'), manual: ['macOS/Linux native validation'] };
  await writeFile(resolve(runRoot,'comparison-gui-results.json'), JSON.stringify(result, null, 2) + '\n');
  console.log(JSON.stringify(result, null, 2));
  await invoke('case_close');
} finally { socket.close(); }
