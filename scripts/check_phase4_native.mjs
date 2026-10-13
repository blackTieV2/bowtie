import assert from 'node:assert/strict';
import {mkdir,writeFile,readFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {createHash} from 'node:crypto';
const port=Number(process.env.BARRIER_ATLAS_CDP_PORT??9335),sleep=ms=>new Promise(r=>setTimeout(r,ms));
let target;
for(let i=0;i<60;i++){try{target=(await(await fetch(`http://127.0.0.1:${port}/json/list`)).json()).find(t=>t.type==='page'&&t.webSocketDebuggerUrl);if(target)break;}catch{}await sleep(250);}assert.ok(target,'Own app diagnostic target did not appear');
const socket=new WebSocket(target.webSocketDebuggerUrl);await new Promise((done,fail)=>{socket.addEventListener('open',done,{once:true});socket.addEventListener('error',fail,{once:true});});
let next=0;const pending=new Map();socket.addEventListener('message',e=>{const m=JSON.parse(String(e.data)),r=pending.get(m.id);if(!r)return;pending.delete(m.id);clearTimeout(r.timer);m.error?r.reject(Error(m.error.message)):r.resolve(m.result);});
function send(method,params={}){return new Promise((resolve,reject)=>{const id=++next,timer=setTimeout(()=>{pending.delete(id);reject(Error(`Timed out ${method}`));},30000);pending.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression){const r=await send('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});assert.ok(!r.exceptionDetails,JSON.stringify(r.exceptionDetails));return r.result.value;}
async function waitFor(expression){for(let i=0;i<150;i++){if(await evaluate(expression))return;await sleep(100);}throw Error(`Condition failed: ${expression}`);}
const invoke=(command,args={})=>evaluate(`window.__TAURI_INTERNALS__.invoke(${JSON.stringify(command)},${JSON.stringify(args)})`);
async function button(text){assert.ok(await evaluate(`(()=>{const b=[...document.querySelectorAll('button')].find(b=>b.textContent.trim()===${JSON.stringify(text)});if(!b||b.disabled)return false;b.click();return true;})()`),text);await sleep(80);}
async function field(selector,value){assert.ok(await evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e)return false;const p=e.tagName==='SELECT'?HTMLSelectElement.prototype:HTMLInputElement.prototype;Object.getOwnPropertyDescriptor(p,'value').set.call(e,${JSON.stringify(value)});e.dispatchEvent(new Event(e.tagName==='SELECT'?'change':'input',{bubbles:true}));return true;})()`),selector);await sleep(60);}
const root=resolve('runs/005-build-phase4'),folder=resolve(process.env.BARRIER_ATLAS_SMOKE_OUTPUT??'.runtime/phase4-walkthrough');
try{
 await waitFor("!!window.__TAURI_INTERNALS__ && document.body.innerText.includes('Barrier Atlas')");
 await invoke('case_open',{path:resolve(folder,'phase3.sqlite')});await send('Page.reload');await waitFor("document.body.innerText.includes('Transfer safety review')");const before=await invoke('case_get');const barrier=before.tree.find(n=>n.title==='Pre-transfer hose inspection');await evaluate(`document.querySelector('[data-entity-id="${barrier.id}"]').click()`);await waitFor("!!document.querySelector('canvas')");
 const hashes={};
 for(const pass of [1,2]){
  const dir=resolve(root,'artifacts',`run-${pass}`);await mkdir(dir,{recursive:true});hashes[`run-${pass}`]={};
  // A fresh frontend context on pass 2 rules out cached bytes. Change only pan/zoom.
  if(pass===2){await send('Page.reload');await waitFor("document.body.innerText.includes('Transfer safety review')");await evaluate(`document.querySelector('[data-entity-id="${barrier.id}"]').click()`);await button('100%');}
  for(const format of ['png','svg','pdf','docx']){
   await button('Export');await field('[name="export-format"]',format);const path=resolve(dir,format==='docx'?'case-report.docx':`diagram.${format}`);await field('[name="export-path"]',path);await button('Export file');await waitFor(`!!document.querySelector('[role=dialog] [role=status]') && document.querySelector('[role=dialog] [role=status]').textContent.includes(${JSON.stringify(path)})`);await button('Close export');const bytes=await readFile(path);assert.ok(bytes.length>100);hashes[`run-${pass}`][format]=createHash('sha256').update(bytes).digest('hex');
  }
 }
 assert.deepEqual(hashes['run-1'],hashes['run-2'],'Exports must be byte identical across fresh frontend runs');assert.deepEqual(await invoke('case_get'),before,'Exporting must not mutate the case');
 const svg=await readFile(resolve(root,'artifacts/run-1/diagram.svg'),'utf8');const p=barrier.properties['canvas.position'];assert.ok(svg.includes(`data-entity-id="${barrier.id}" transform="translate(${p.x} ${p.y})"`),'SVG preserves the dragged barrier position');
 const regression=JSON.parse(await readFile(resolve(process.env.BARRIER_ATLAS_PHASE3_RUN,'validation-results.json'),'utf8'));assert.equal(regression.result,'PASS');
 await mkdir(resolve(root,'evidence'),{recursive:true});const screenshot=await send('Page.captureScreenshot',{format:'png'});await writeFile(resolve(root,'evidence/export-workspace.png'),Buffer.from(screenshot.data,'base64'));
 const result={result:'PASS',scope:'Windows native GUI exports from live walkthrough case',phase3Regression:'PASS',twelveSteps:regression.twelveSteps,entities:before.tree.length,formats:['png','svg','pdf','docx'],twoFreshFrontendRuns:true,byteIdentical:true,unchangedCase:true,savedPositionVerified:true,hashes,manual:['Native Browse save chooser','macOS/Linux native validation','Independent QA','Cross-platform raster equivalence']};await writeFile(resolve(root,'gui-validation-results.json'),JSON.stringify(result,null,2)+'\n');console.log(JSON.stringify(result,null,2));await invoke('case_close');
}finally{socket.close();}
