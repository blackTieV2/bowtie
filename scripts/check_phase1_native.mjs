// Tests the project's built WebView2 app through a local diagnostic CDP port.
// Start only this app with WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9331.
import { mkdir } from "node:fs/promises";
import { resolve } from "node:path";
import assert from "node:assert/strict";

const port = Number(process.env.BARRIER_ATLAS_CDP_PORT ?? "9331");
const sleep = ms => new Promise(done => setTimeout(done, ms));
let target;
for (let attempt = 0; attempt < 40; attempt++) {
  try {
    const targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
    target = targets.find(item => item.type === "page" && item.webSocketDebuggerUrl);
    if (target) break;
  } catch { /* Wait for our own app's WebView2 startup. */ }
  await sleep(250);
}
assert.ok(target, "The app's local diagnostic WebView target did not appear");
const socket = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((done, fail) => { socket.addEventListener("open", done, { once: true }); socket.addEventListener("error", fail, { once: true }); });
let nextId = 0;
const pending = new Map();
socket.addEventListener("message", event => {
  const message = JSON.parse(String(event.data));
  const request = pending.get(message.id);
  if (!request) return;
  pending.delete(message.id); clearTimeout(request.timeout);
  if (message.error) request.reject(new Error(message.error.message)); else request.resolve(message.result);
});
function send(method, params = {}) {
  return new Promise((resolve, reject) => {
    const id = ++nextId;
    const timeout = setTimeout(() => { pending.delete(id); reject(new Error(`Timed out: ${method}`)); }, 10000);
    pending.set(id, { resolve, reject, timeout });
    socket.send(JSON.stringify({ id, method, params }));
  });
}
async function evaluate(expression) {
  const result = await send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
  assert.ok(!result.exceptionDetails, JSON.stringify(result.exceptionDetails));
  return result.result.value;
}
async function waitFor(expression) {
  for (let attempt = 0; attempt < 40; attempt++) {
    if (await evaluate(expression)) return;
    await sleep(100);
  }
  throw new Error(`Condition did not become true: ${expression}`);
}
const invoke = (command, args = {}) => evaluate(`window.__TAURI_INTERNALS__.invoke(${JSON.stringify(command)}, ${JSON.stringify(args)})`);
const refresh = async text => { await send("Page.reload"); await waitFor(`document.body.innerText.includes(${JSON.stringify(text)})`); };
try {
  await send("Runtime.enable");
  await waitFor("!!window.__TAURI_INTERNALS__ && document.body.innerText.includes('Barrier Atlas')");
  assert.equal(await invoke("case_get"), null);
  const created = await invoke("case_create", { title: "Native smoke case", organization: "Example workshop" });
  const grouped = await invoke("case_add_group", { parentId: created.tree[0].id, title: "Native group" });
  const group = grouped.tree.find(row => row.kind === "group");
  const diagram = await invoke("case_add_diagram", { parentId: group.id, hazardTitle: "Transfer operation", topEventTitle: "Containment lost" });
  assert.equal(diagram.tree.length, 5);
  await refresh("Containment lost");
  assert.ok(await evaluate("document.querySelector('aside').innerText.includes('Native group')"));
  await evaluate("[...document.querySelectorAll('button')].find(button => button.textContent==='Close case').click()");
  await waitFor("document.body.innerText.includes('Save your changes?')");
  await evaluate("[...document.querySelectorAll('[role=dialog] button')].find(button => button.textContent==='Cancel').click()");
  assert.equal((await invoke("case_get")).dirty, true);
  const folder = resolve(".runtime/native-smoke");
  await mkdir(folder, { recursive: true });
  const path = resolve(folder, "case.sqlite");
  assert.equal((await invoke("case_save", { path })).dirty, false);
  await invoke("case_close");
  assert.equal(await invoke("case_get"), null);
  const reopened = await invoke("case_open", { path });
  assert.deepEqual(reopened.tree, diagram.tree);
  assert.equal(reopened.dirty, false);
  await refresh("Containment lost");
  assert.ok(await evaluate("document.body.innerText.includes('Native smoke case')"));
  console.log(JSON.stringify({ result: "PASS", scope: "Windows built-app IPC and explorer smoke", entities: 5, saveReopen: true, sqliteTreeRendered: true, unsavedClosePrompt: true, phase2Canvas: false }, null, 2));
  await invoke("case_close");
} finally {
  socket.close();
}
