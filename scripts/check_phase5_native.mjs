// Phase 5 acceptance through the repository's own native app controls.
import assert from 'node:assert/strict';
import { mkdir, readFile, writeFile, copyFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { createHash } from 'node:crypto';
const sleep=ms=>new Promise(r=>setTimeout(r,ms));
const port=Number(process.env.BARRIER_ATLAS_CDP_PORT??9336);
let target;
for(let i=0;i<120;i++){try{target=(await(await fetch(`http://127.0.0.1:${port}/json/list`)).json()).find(t=>t.type==='page'&&t.webSocketDebuggerUrl);if(target)break;}catch{}await sleep(250);}
assert.ok(target,'Own app diagnostic target did not appear');
const socket=new WebSocket(target.webSocketDebuggerUrl);
await new Promise((done,fail)=>{socket.addEventListener('open',done,{once:true});socket.addEventListener('error',fail,{once:true});});
let next=0;const pending=new Map();
socket.addEventListener('message',e=>{const m=JSON.parse(String(e.data)),r=pending.get(m.id);if(!r)return;pending.delete(m.id);clearTimeout(r.timer);m.error?r.reject(Error(m.error.message)):r.resolve(m.result);});
function send(method,params={}){return new Promise((resolve,reject)=>{const id=++next,timer=setTimeout(()=>{pending.delete(id);reject(Error(`Timed out ${method}`));},30000);pending.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression){const r=await send('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});assert.ok(!r.exceptionDetails,JSON.stringify(r.exceptionDetails));return r.result.value;}
async function waitFor(expression){for(let i=0;i<150;i++){if(await evaluate(expression))return;await sleep(100);}throw Error(`Condition failed: ${expression}`);}
const invoke=(command,args={})=>evaluate(`window.__TAURI_INTERNALS__.invoke(${JSON.stringify(command)},${JSON.stringify(args)})`);
async function button(text){assert.ok(await evaluate(`(()=>{const b=[...document.querySelectorAll('button')].find(b=>b.textContent.trim()===${JSON.stringify(text)});if(!b||b.disabled)return false;b.click();return true;})()`),text);await sleep(80);}
async function field(selector,value){assert.ok(await evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e)return false;const p=e.tagName==='TEXTAREA'?HTMLTextAreaElement.prototype:e.tagName==='SELECT'?HTMLSelectElement.prototype:HTMLInputElement.prototype;Object.getOwnPropertyDescriptor(p,'value').set.call(e,${JSON.stringify(value)});e.dispatchEvent(new Event(e.tagName==='SELECT'?'change':'input',{bubbles:true}));return true;})()`),selector);await sleep(60);}
const root=resolve(process.env.BARRIER_ATLAS_PHASE5_RUN??'runs/006-build-phase5'),folder=resolve(process.env.BARRIER_ATLAS_SMOKE_OUTPUT??'.runtime/phase5-walkthrough');
const model=s=>{const {path,dirty,...data}=s;return data;};
async function reload(){await send('Page.reload');await waitFor("!!window.__TAURI_INTERNALS__ && !!document.querySelector('.workspace-tabs')");}
async function reopenEqual(path){const before=await invoke('case_get');await invoke('case_save',{path});await invoke('case_close');const after=await invoke('case_open',{path});assert.deepEqual(model(after),model(before),'Save/reopen model equality');await reload();return after;}
async function screenshot(name){const capture=await send('Page.captureScreenshot',{format:'png'});await writeFile(resolve(root,'evidence',name),Buffer.from(capture.data,'base64'));}
try{
 await mkdir(resolve(root,'artifacts'),{recursive:true});await mkdir(resolve(root,'evidence'),{recursive:true});
 await waitFor("!!window.__TAURI_INTERNALS__ && document.body.innerText.includes('Barrier Atlas')");
 const casePath=resolve(folder,'phase3.sqlite');await invoke('case_open',{path:casePath});await reload();
 const reference=resolve(folder,'compare-reference.sqlite');await copyFile(casePath,reference);const referenceBytes=await readFile(reference);
 const before=await invoke('case_get');const preview=await invoke('case_replace_preview',{find:'Pre-transfer',replacement:'Scheduled'});assert.ok(preview.changes.length>0);
 await button('Find & replace');await field('[name="replace-find"]','Pre-transfer');await field('[name="replace-with"]','Scheduled');await button('Preview replacements');
 await waitFor("!!document.querySelector('.diff-table tbody tr')");assert.equal(await evaluate("document.querySelectorAll('.diff-table tbody tr').length"),preview.changes.length);
 for(const change of preview.changes)assert.ok(await evaluate(`document.body.innerText.includes(${JSON.stringify(change.path)})`));
 assert.deepEqual(await invoke('case_get'),before);await screenshot('find-replace-preview.png');
 await button('Replace all previewed fields');await waitFor("document.body.innerText.includes('Replaced ')");assert.equal((await invoke('case_replace_preview',{find:'Pre-transfer',replacement:'Scheduled'})).changes.length,0);
 const replaced=await reopenEqual(casePath);assert.ok(replaced.tree.some(n=>n.title.includes('Scheduled')));
 await button('Compare cases');await field('[name="comparison-path"]',reference);await button('Compare');await waitFor("!!document.querySelector('.diff-table tbody tr')");
 const reportPath=resolve(root,'artifacts/case-differences.txt');await field('[name="comparison-report-path"]',reportPath);await button('Save diff report');await waitFor("document.body.innerText.includes('Diff report saved:')");
 assert.deepEqual(await invoke('case_get'),replaced);assert.deepEqual(await readFile(reference),referenceBytes);await screenshot('full-case-comparison.png');await reopenEqual(casePath);
 const expected=(await invoke('case_table_export')).text;const csvPath=resolve(root,'artifacts/case-data.csv');
 await button('CSV & spreadsheet');await field('[name="table-path"]',csvPath);await button('Export case table');await waitFor("document.body.innerText.includes('Table exported:')");assert.equal(await readFile(csvPath,'utf8'),expected);
 await invoke('case_create',{title:'Import target',organization:'New organization'});await reload();await button('CSV & spreadsheet');await field('[name="table-path"]',csvPath);await button('Read table');await waitFor("document.querySelector('[name=table-content]').value.length>10");
 await button('Validate & preview import');await waitFor("document.body.innerText.includes('Validation passed.')");await screenshot('csv-dry-run.png');await button('Apply validated import');await waitFor("document.body.innerText.includes('Import applied:')");
 assert.equal((await invoke('case_table_export')).text,expected);const importedPath=resolve(folder,'imported.sqlite');await reopenEqual(importedPath);assert.equal((await invoke('case_table_export')).text,expected);
 const reexport=resolve(root,'artifacts/case-data-reexport.csv');await button('CSV & spreadsheet');await field('[name="table-path"]',reexport);await button('Export case table');await waitFor("document.body.innerText.includes('Table exported:')");assert.deepEqual(await readFile(reexport),await readFile(csvPath));
 const xlsx=resolve(root,'artifacts/case-data.xlsx');await field('[name="table-path"]',xlsx);await button('Export case table');await waitFor("document.body.innerText.includes('Table exported:')");
 const xlsx2=resolve(folder,'repeat.xlsx');await field('[name="table-path"]',xlsx2);await button('Export case table');await waitFor("document.body.innerText.includes('Table exported:')");assert.deepEqual(await readFile(xlsx),await readFile(xlsx2));
 await invoke('case_create',{title:'Spreadsheet target',organization:'New organization'});await reload();await button('CSV & spreadsheet');await field('[name="table-path"]',xlsx);await button('Read table');await waitFor("!!document.querySelector('[name=table-sheet]')");assert.equal(await evaluate("document.querySelector('[name=table-content]').value"),expected.replaceAll('\r\n','\n'));
 await button('Validate & preview import');await waitFor("document.body.innerText.includes('Validation passed.')");await button('Apply validated import');await waitFor("document.body.innerText.includes('Import applied:')");await reopenEqual(resolve(folder,'spreadsheet-imported.sqlite'));assert.equal((await invoke('case_table_export')).text,expected);
 const unchanged=await invoke('case_get'),savedBytes=await readFile(unchanged.path);await button('CSV & spreadsheet');await field('[name="table-content"]','record_type,"unclosed');await button('Validate & preview import');await waitFor("document.body.innerText.includes('Import rejected.')");assert.deepEqual(await invoke('case_get'),unchanged);assert.deepEqual(await readFile(unchanged.path),savedBytes);
 const existingRatings=(await invoke('case_get')).lookups.filter(l=>l.tableKey==='effectiveness');for(const [i,lookup] of existingRatings.entries())await invoke('case_save_lookup',{lookup:{...lookup,sortOrder:(i+1)*10}});await reload();
 await button('Property choices');await button('New choice');assert.equal(Number(await evaluate("document.querySelector('[name=choice-order]').value")),existingRatings.length*10+1);await field('[name="choice-name"]','Additional choice');await field('[name="choice-code"]','NEW');await button('Save choice');await waitFor("!document.querySelector('[role=dialog]')");
 const ratingBefore=await invoke('case_get');const values=ratingBefore.lookups.filter(l=>l.tableKey==='effectiveness');assert.ok(values.length>1);const id=values[0].id;
 assert.ok(await evaluate("(()=>{const b=[...document.querySelectorAll('.matrix-summary button')].find(b=>b.textContent==='Move down'&&!b.disabled);if(!b)return false;b.click();return true;})()"));await waitFor("!document.body.innerText.includes('Working…')");const ratingAfter=await invoke('case_get');assert.equal(ratingAfter.lookups.filter(l=>l.tableKey==='effectiveness')[1].id,id);assert.deepEqual(ratingBefore.tree,ratingAfter.tree);await reopenEqual(resolve(folder,'lookup-reordered.sqlite'));
 const regression=JSON.parse(await readFile(resolve(process.env.BARRIER_ATLAS_PHASE3_RUN,'validation-results.json'),'utf8'));assert.equal(regression.result,'PASS');
 const artifactHashes={};for(const name of ['case-data.csv','case-data-reexport.csv','case-data.xlsx','case-differences.txt'])artifactHashes[name]=createHash('sha256').update(await readFile(resolve(root,'artifacts',name))).digest('hex');
 const result={result:'PASS',scope:'Windows native Phase 5 acceptance',twelveSteps:regression.twelveSteps,phase3Regression:'PASS',findReplacePreviewPaths:true,findReplaceApplyCount:true,findReplaceSaveReopen:true,twoCaseCompareReadOnly:true,compareSaveReopen:true,csvGuiRoundTrip:true,csvStructuralByteEquality:true,csvSaveReopen:true,xlsxGuiRoundTrip:true,xlsxDeterministicBytes:true,xlsxSaveReopen:true,rejectionByteEquality:true,lookupNewChoiceAppendsAfterSparseOrders:true,lookupReorderPreservesAssignments:true,lookupSaveReopen:true,artifactHashes,manual:['Native Browse chooser','macOS/Linux native execution']};
 await writeFile(resolve(root,'gui-validation-results.json'),JSON.stringify(result,null,2)+'\n');console.log(JSON.stringify(result,null,2));await invoke('case_close');
}finally{socket.close();}
