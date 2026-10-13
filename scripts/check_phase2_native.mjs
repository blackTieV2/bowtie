// Original built-app GUI walkthrough. Only persistence/path setup uses direct IPC;
// every entity, property and typed link is authored through rendered GUI controls.
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import assert from "node:assert/strict";
const port=Number(process.env.BARRIER_ATLAS_CDP_PORT??"9333");
const sleep=ms=>new Promise(done=>setTimeout(done,ms));
let target;
for(let i=0;i<60;i++) {
  try { target=(await (await fetch(`http://127.0.0.1:${port}/json/list`)).json()).find(t=>t.type==="page" && t.webSocketDebuggerUrl);if(target)break; }catch{}
  await sleep(250);
}
assert.ok(target,"Own app WebView diagnostic target missing");
const socket=new WebSocket(target.webSocketDebuggerUrl);
await new Promise((done,fail)=>{socket.addEventListener("open",done,{once:true});socket.addEventListener("error",fail,{once:true});});
let next=0;const requests=new Map();
socket.addEventListener("message",e=>{const m=JSON.parse(String(e.data)),r=requests.get(m.id);if(!r)return;requests.delete(m.id);clearTimeout(r.timer);m.error?r.reject(Error(m.error.message)):r.resolve(m.result);});
function send(method,params={}) {return new Promise((resolve,reject)=>{const id=++next,timer=setTimeout(()=>{requests.delete(id);reject(Error(`Timed out ${method}`));},15000);requests.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression) {const r=await send("Runtime.evaluate",{expression,awaitPromise:true,returnByValue:true});assert.ok(!r.exceptionDetails,JSON.stringify(r.exceptionDetails));return r.result.value;}
async function waitFor(expression) {for(let i=0;i<60;i++){if(await evaluate(expression))return;await sleep(100);}throw Error(`GUI condition failed: ${expression}`);}
const invoke=(command,args={})=>evaluate(`window.__TAURI_INTERNALS__.invoke(${JSON.stringify(command)},${JSON.stringify(args)})`);
const snapshot=()=>invoke("case_get");
async function button(text,scope="document") {
  const found=await evaluate(`(()=>{const b=[...${scope}.querySelectorAll('button')].find(b=>b.textContent.trim()===${JSON.stringify(text)});if(!b||b.disabled)return false;b.click();return true;})()`);
  assert.ok(found,`Enabled GUI button ${text} missing`);await sleep(60);
}
async function field(selector,value) {
  assert.ok(await evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e)return false;const p=e.tagName==='TEXTAREA'?HTMLTextAreaElement.prototype:e.tagName==='SELECT'?HTMLSelectElement.prototype:HTMLInputElement.prototype;Object.getOwnPropertyDescriptor(p,'value').set.call(e,${JSON.stringify(value)});e.dispatchEvent(new Event(e.tagName==='SELECT'?'change':'input',{bubbles:true}));return true;})()`),`GUI field missing: ${selector}`);
}
async function select(title) {
  const s=await snapshot(),node=s.tree.find(n=>n.title===title && n.kind!=="diagram");assert.ok(node,title);
  await evaluate(`document.querySelector('[data-entity-id="${node.id}"]').click()`);await sleep(60);return node;
}
async function create(buttonTitle,title,values={}) {
  await button(buttonTitle);await waitFor("!!document.querySelector('[role=dialog] input[name=entity-title]')");
  await field('[name="entity-title"]',title);
  for(const [key,value] of Object.entries(values))await field(`[name="${key}"]`,value);
  await button("Create","document.querySelector('[role=dialog]')");
  await waitFor("!document.querySelector('[role=dialog]')");
  const s=await snapshot();assert.ok(s.tree.some(n=>n.title===title),`GUI did not persist ${title}`);return s.tree.find(n=>n.title===title);
}
async function link(source,relation,targetTitle) {
  await select(source);await field('[aria-label="Relationship"]',relation);await sleep(60);
  const target=(await snapshot()).tree.find(n=>n.title===targetTitle);assert.ok(target);
  await field('[aria-label="Link target"]',target.id);await button("Link");
  await waitFor(`document.querySelector('.links').innerText.includes(${JSON.stringify(targetTitle)})`);
}
async function mouse(type,x,y,extra={}) {await send("Input.dispatchMouseEvent",{type,x,y,...extra});}
async function drag(from,to) {
  await mouse("mouseMoved",from.x,from.y);await mouse("mousePressed",from.x,from.y,{button:"left",clickCount:1});
  for(let i=1;i<=8;i++){await mouse("mouseMoved",from.x+(to.x-from.x)*i/8,from.y+(to.y-from.y)*i/8,{button:"left",buttons:1});await sleep(25);}
  await mouse("mouseReleased",to.x,to.y,{button:"left",clickCount:1});await sleep(200);
}
const folder=resolve(process.env.BARRIER_ATLAS_SMOKE_OUTPUT??".runtime/phase2-walkthrough");await mkdir(folder,{recursive:true});
const steps=[];
try {
  await send("Runtime.enable");await waitFor("!!window.__TAURI_INTERNALS__ && document.body.innerText.includes('Barrier Atlas')");
  assert.equal(await snapshot(),null,"Use a fresh app session");
  await button("New case");await field('[role=dialog] input',"Transfer safety review");await field('[role=dialog] label:nth-of-type(2) input',"Example workshop");await button("Create","document.querySelector('[role=dialog]')");await waitFor("!document.querySelector('[role=dialog]')");
  await button("Add group");await field('[role=dialog] input',"Transfer station");await button("Create","document.querySelector('[role=dialog]')");await waitFor("!document.querySelector('[role=dialog]')");
  await button("Add diagram");await field('[role=dialog] input',"Flammable liquid transfer");await field('[role=dialog] label:nth-of-type(2) input',"Loss of containment");await button("Create","document.querySelector('[role=dialog]')");await waitFor("!!document.querySelector('canvas') && !document.querySelector('[role=dialog]')");steps.push(1,2);
  await create("Add threat","Hose rupture");await select("Loss of containment");await create("Add threat","Tank overfill");steps.push(3);
  await select("Loss of containment");await create("Add consequence","Pool fire");await select("Loss of containment");await create("Add consequence","Environmental release");steps.push(4);
  await select("Hose rupture");await create("Add barrier","Pre-transfer hose inspection");await select("Tank overfill");await create("Add barrier","High-level trip");steps.push(5);
  await select("Pool fire");await create("Add barrier","Emergency isolation");await select("Environmental release");await create("Add barrier","Bund containment");steps.push(6);
  await select("Pre-transfer hose inspection");await create("Add escalation factor","Poor lighting");steps.push(7);
  await create("Add barrier","Portable inspection light");steps.push(8);
  await create("New activity","Inspect transfer hose",{timing:"Before each transfer",instructions:"Use the hose condition checklist",verification:"Supervisor checks the signed inspection record"});
  await link("Pre-transfer hose inspection","activity","Inspect transfer hose");steps.push(9);
  await create("New job title","Transfer operator");await create("New document","Hose condition checklist",{reference:"procedures/hose-checklist.txt"});
  await link("Inspect transfer hose","responsible","Transfer operator");await link("Inspect transfer hose","document","Hose condition checklist");await link("Pre-transfer hose inspection","accountable","Transfer operator");steps.push(10);
  await create("New product","Signed hose inspection record",{reference:"records/inspection.txt"});await link("Inspect transfer hose","output","Signed hose inspection record");steps.push(11);
  await select("Pre-transfer hose inspection");await create("Add action","Improve inspection lighting",{alarp_rationale:"Install fixed lighting; a practical improvement to inspection reliability. Review by the responsible team is required."});
  await select("Flammable liquid transfer");await button("Edit");await field('[name="alarp_rationale"]',"Review further practicable safeguards with the team; this diagram does not certify safety.");await button("Apply","document.querySelector('[role=dialog]')");await waitFor("!document.querySelector('[role=dialog]')");steps.push(12);
  // Edit and successful leaf deletion through the same GUI.
  await select("Tank overfill");await button("Edit");await field('[name="entity-title"]',"Tank overfill during transfer");await button("Apply","document.querySelector('[role=dialog]')");await waitFor("!document.querySelector('[role=dialog]')");
  await select("Loss of containment");await create("Add threat","Temporary review item");await button("Delete");await button("Delete item","document.querySelector('[role=dialog]')");await waitFor("!document.querySelector('[role=dialog]')");assert.ok(!(await snapshot()).tree.some(n=>n.title==="Temporary review item"));
  // RESTRICT must identify the actual child and incoming shared link in the GUI.
  await select("Hose rupture");await button("Delete");await button("Delete item","document.querySelector('[role=dialog]')");await waitFor("document.querySelector('[role=dialog]').innerText.includes('child barrier: Pre-transfer hose inspection')");await button("Cancel","document.querySelector('[role=dialog]')");
  await select("Inspect transfer hose");await button("Delete");await button("Delete item","document.querySelector('[role=dialog]')");await waitFor("document.querySelector('[role=dialog]').innerText.includes('incoming activity link: Pre-transfer hose inspection')");await button("Cancel","document.querySelector('[role=dialog]')");
  await select("Pre-transfer hose inspection");await button("Auto-layout");await waitFor("document.querySelector('footer').innerText.startsWith('Offline')");await button("Fit");
  const laidOut=await snapshot(),topPosition=laidOut.tree.find(n=>n.kind==='top_event').properties['canvas.position'];
  for(const n of laidOut.tree.filter(n=>['threat','consequence'].includes(n.kind))) {
    const p=n.properties['canvas.position'];assert.ok(n.kind==='threat'?p.x<topPosition.x:p.x>topPosition.x,'Classic geometry side');
    for(const b of laidOut.tree.filter(b=>b.kind==='barrier' && b.parentId===n.id)) {
      const q=b.properties['canvas.position'];assert.ok(q.x>Math.min(p.x,topPosition.x) && q.x<Math.max(p.x,topPosition.x),'Barrier lies on its path');assert.equal(q.y,p.y);
    }
  }
  const factor=laidOut.tree.find(n=>n.kind==='escalation_factor'),owner=laidOut.tree.find(n=>n.id===factor.parentId);
  assert.ok(factor.properties['canvas.position'].y>owner.properties['canvas.position'].y,'Escalation factor lies below its barrier');
  const beforeDrag=(await snapshot()).tree.find(n=>n.title==="Pre-transfer hose inspection");
  const transform=await evaluate("(()=>{const e=document.querySelector('.canvas-host');e.scrollIntoView({block:'center'});const r=e.getBoundingClientRect();return {left:r.left,top:r.top,x:Number(e.dataset.viewX),y:Number(e.dataset.viewY),scale:Number(e.dataset.viewScale)};})()");
  const p=beforeDrag.properties["canvas.position"],screen={x:transform.left+transform.x+(p.x+85)*transform.scale,y:transform.top+transform.y+(p.y+39)*transform.scale};
  await drag(screen,{x:screen.x+24,y:screen.y+30});
  const afterDrag=(await snapshot()).tree.find(n=>n.id===beforeDrag.id);assert.ok(afterDrag.properties["canvas.position"].x>p.x+10,"Canvas pointer drag did not persist");
  const zoomBefore=await evaluate("Number(document.querySelector('.canvas-host').dataset.viewScale)");await evaluate("document.querySelector('[aria-label=\"Zoom in\"]').click()");await sleep(80);assert.ok(await evaluate(`Number(document.querySelector('.canvas-host').dataset.viewScale)>${zoomBefore}`));
  const panBefore=await evaluate("Number(document.querySelector('.canvas-host').dataset.viewX)");const host=await evaluate("(()=>{const r=document.querySelector('.canvas-host').getBoundingClientRect();return {x:r.left+8,y:r.top+8};})()");await drag(host,{x:host.x+25,y:host.y+18});assert.notEqual(await evaluate("Number(document.querySelector('.canvas-host').dataset.viewX)"),panBefore);
  const wheelZoom=await evaluate("Number(document.querySelector('.canvas-host').dataset.viewScale)");
  await mouse('mouseWheel',host.x+50,host.y+50,{deltaX:0,deltaY:-120,modifiers:2});await sleep(80);assert.ok(await evaluate(`Number(document.querySelector('.canvas-host').dataset.viewScale)>${wheelZoom}`));
  const wheelPan=await evaluate("Number(document.querySelector('.canvas-host').dataset.viewY)");
  await mouse('mouseWheel',host.x+50,host.y+50,{deltaX:0,deltaY:40});await sleep(80);assert.notEqual(await evaluate("Number(document.querySelector('.canvas-host').dataset.viewY)"),wheelPan);
  await button("Fit");
  const final=await snapshot();
  const counts=Object.fromEntries([...new Set(final.tree.map(n=>n.kind))].map(k=>[k,final.tree.filter(n=>n.kind===k).length]));
  assert.deepEqual(counts,{organization:1,group:1,hazard:1,diagram:1,top_event:1,activity:1,document:1,job_title:1,product:1,threat:2,consequence:2,barrier:5,action:1,escalation_factor:1});
  assert.equal(final.links.length,5);
  const path=resolve(folder,"twelve-steps.sqlite");await invoke("case_save",{path});
  await send("Page.reload");await waitFor("document.body.innerText.includes('Transfer safety review')");
  await select("Loss of containment");await button("Save");await waitFor("document.querySelector('footer').innerText.startsWith('Offline')");await button("Close case");await waitFor("document.body.innerText.includes('No case open')");
  const reopened=await invoke("case_open",{path});assert.deepEqual(reopened.tree,final.tree);assert.deepEqual(reopened.links,final.links);assert.equal(reopened.dirty,false);
  await send("Page.reload");await waitFor("document.body.innerText.includes('Transfer safety review')");await select("Pre-transfer hose inspection");await waitFor("!!document.querySelector('canvas')");
  const image=await send("Page.captureScreenshot",{format:"png"});await writeFile(resolve(folder,"canvas.png"),Buffer.from(image.data,"base64"));
  const result={result:"PASS",scope:"Windows built-app GUI authoring and pointer interactions",steps,counts,links:5,classicGeometry:true,canvasDragPersisted:true,zoomPan:true,ctrlWheelZoom:true,wheelPan:true,restrictDiagnostics:true,editDelete:true,saveReopenStructure:true,manual:["Native Save As/Open file chooser path selection (test supplies its own path by IPC)","Human methodology/ALARP judgment","macOS/Linux native execution","Independent QA"]};
  await writeFile(resolve(process.env.BARRIER_ATLAS_RESULT_PATH??"runs/003-build-phase2/validation-results.json"),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify(result,null,2));await invoke("case_close");
} finally {socket.close();}
