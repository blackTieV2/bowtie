// Phase 6 acceptance through the repository's own native app controls.
import assert from 'node:assert/strict';
import { mkdir, readFile, writeFile, copyFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { createRequire } from 'node:module';
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
const {unzipSync,zipSync,strFromU8,strToU8}=createRequire(resolve('app/package.json'))('fflate');
const runRoot=resolve(process.env.BARRIER_ATLAS_PHASE6_RUN??'runs/007-build-phase6'),folder=resolve(process.env.BARRIER_ATLAS_SMOKE_OUTPUT??'.runtime/phase6-walkthrough');
const strip=s=>{const {path,dirty,...data}=s;return data;};
async function reload(){await send('Page.reload');await waitFor("!!document.querySelector('.workspace-tabs')");}
async function check(selector,checked=true){assert.ok(await evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e)return false;if(e.checked!==${checked})e.click();return true;})()`),selector);await sleep(60);}
async function screenshot(name){const r=await send('Page.captureScreenshot',{format:'png'});await writeFile(resolve(runRoot,'evidence',name),Buffer.from(r.data,'base64'));}
async function reopenEqual(path){const before=await invoke('case_get');await invoke('case_save',{path});await invoke('case_close');const after=await invoke('case_open',{path});assert.deepEqual(strip(after),strip(before));await reload();return after;}
async function makeQuestion(title,answers,targets,parentId=null,contextual=false,mostNegative=false,positiveFirst=true){
 await button('New audit question');await field('[name="audit-question-title"]',title);await field('[name="audit-question-answers"]',answers);await field('[name="audit-question-criteria"]','Review the transfer checklist and observation record');if(parentId)await field('[name="audit-question-parent"]',parentId);
 const boxes=await evaluate("[...document.querySelectorAll('[data-audit-target]')].map(e=>e.dataset.auditTarget)");for(const id of boxes)await check(`[data-audit-target="${id}"]`,targets.includes(id));
 await check('[name="audit-contextual"]',contextual);await check('[name="audit-most-negative"]',mostNegative);await check('[name="audit-positive-first"]',positiveFirst);await button('Save question');await waitFor("!document.querySelector('[role=dialog]')");return(await invoke('case_get')).audits[0].questions.find(q=>q.title===title).id;
}
try{
 await mkdir(resolve(runRoot,'artifacts'),{recursive:true});await mkdir(resolve(runRoot,'evidence'),{recursive:true});
 await waitFor("!!window.__TAURI_INTERNALS__ && document.body.innerText.includes('Barrier Atlas')");
 await invoke('case_open',{path:resolve(folder,'phase3.sqlite')});await reload();const original=await invoke('case_get');
 const barrier=original.tree.find(n=>n.title==='Pre-transfer hose inspection'),second=original.tree.find(n=>n.title==='Emergency isolation'),activity=original.tree.find(n=>n.title==='Inspect transfer hose');assert.ok(barrier&&second&&activity);
 await evaluate(`document.querySelector('[data-entity-id="${barrier.id}"]').click()`);await button('Audits');await button('New audit');await field('[name="audit-name"]','Transfer assurance');await button('Save audit');await waitFor("!document.querySelector('[role=dialog]')");
 const parent=await makeQuestion('Review framework','Established\nNeeds attention',[]);
 const safeguard=await makeQuestion('Check safeguard implementation','Verified\nNeeds review\nAbsent',[barrier.id,second.id],parent,true,true,true);
 const management=await makeQuestion('Check inspection work','Missing\n@Ready\nVerified',[activity.id],null,false,false,false);
 await button('New audit survey');await field('[name="audit-survey-code"]','TR-01');await field('[name="audit-survey-title"]','Liquid-transfer checks');await field('[name="audit-survey-description"]','Barrier implementation and supporting work');
 for(const qid of [parent,safeguard,management])await check(`[data-survey-question="${qid}"]`,qid!==parent);await button('Save survey');await waitFor("!document.querySelector('[role=dialog]')");
 await button('Conduct survey');await field('[name="audit-fillout-date"]','2026-10-13');await field('[name="audit-fillout-respondent"]','Local reviewer');await field('[name="audit-fillout-unit"]','Operations');
 const answers=await evaluate("[...document.querySelectorAll('select[name^=audit-answer-]')].map(e=>e.name)");assert.equal(answers.length,3);
 for(const [i,name] of answers.entries()){await field(`[name="${name}"]`,i===1?'2':'0');await field(`[name="audit-remark-${i}"]`,'Observed during transfer; checklist reference recorded');}
 await check('[name="audit-fillout-complete"]');await screenshot('survey-execution.png');await button('Save fill-out');await waitFor("!document.querySelector('[role=dialog]')");
 let model=await invoke('case_get');assert.equal(model.audits[0].surveys[0].fillouts.length,1);assert.equal(model.audits[0].surveys[0].fillouts[0].completed,true);
 const formPath=resolve(runRoot,'artifacts/audit-form.xlsx');await field('[name="audit-output-path"]',formPath);await button('Export audit form');await waitFor("document.body.innerText.includes('Audit form exported:')");
 // Independently author an Excel-like response: calendar date serial + built-in date
 // style, not the application's own importer or workbook writer.
 const parts=unzipSync(await readFile(formPath));let sheet=strFromU8(parts['xl/worksheets/sheet1.xml']);
 const xml=s=>s.replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;');
 const slots=model.audits[0].surveys[0].fillouts[0].answers;
 const serial=Math.round((Date.UTC(2026,9,14)-Date.UTC(1899,11,30))/86400000);
 for(const [i,slot] of slots.entries()){
  const number=i+2,q=model.audits[0].questions.find(q=>q.id===slot.questionId),choices=q.answers.filter(a=>a!=='@').map(a=>a.startsWith('@')?a.slice(1):a);
  const values={H:choices[0],J:'External reviewer',K:'Night shift',L:'Independent Excel-like response evidence'};
  for(const [column,value] of Object.entries(values))sheet=sheet.replace(new RegExp(`<c r="${column}${number}"[^>]*>.*?<\\/c>`),`<c r="${column}${number}" s="1" t="inlineStr"><is><t>${xml(value)}</t></is></c>`);
  sheet=sheet.replace(new RegExp(`<c r="I${number}"[^>]*>.*?<\\/c>`),`<c r="I${number}" s="2"><v>${serial}</v></c>`);
 }
 parts['xl/worksheets/sheet1.xml']=strToU8(sheet);parts['xl/styles.xml']=strToU8(strFromU8(parts['xl/styles.xml']).replace('<cellXfs count="2">','<cellXfs count="3">').replace('</cellXfs>','<xf numFmtId="14" fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/></cellXfs>'));
 const filled=resolve(runRoot,'artifacts/audit-filled.xlsx');await writeFile(filled,zipSync(parts,{level:6,mtime:new Date(1980,0,1)}));await field('[name="audit-output-path"]',filled);await button('Import audit forms');await waitFor("document.body.innerText.includes('Imported 1 fill-out forms.')");
 model=await invoke('case_get');const a=model.audits[0];assert.equal(a.surveys[0].fillouts.length,2);assert.equal(a.surveys[0].fillouts[1].date,'2026-10-14');
 const row=model.auditResults[a.id].find(r=>r.questionId===safeguard);assert.deepEqual(row.counts,[3,0,1]);assert.equal(row.worstChoice,2);await screenshot('audit-results.png');
 await field('[name="audit-filter-unit"]','Operations');await field('[name="audit-filter-name"]','Operations review');await button('Save audit filter');await waitFor("!document.body.innerText.includes('Working…')");
 await field('[name="audit-filter-unit"]','');const report=resolve(runRoot,'artifacts/audit-review.docx');await field('[name="audit-output-path"]',report);await button('Export audit review');await waitFor("document.body.innerText.includes('Audit review exported:')");
 const casePath=resolve(folder,'audited.sqlite');model=await reopenEqual(casePath);await button('Audits');const saved=model.audits[0].savedFilters.find(p=>p.name==='Operations review');assert.ok(saved);await field('[name="audit-saved-filter"]',saved.id);assert.equal(await evaluate("document.querySelector('[name=audit-filter-unit]').value"),'Operations');
 // Display questions and click graph bar for respondent details; persist diagram filter.
 await button('Diagram');await evaluate(`document.querySelector('[data-entity-id="${barrier.id}"]').click()`);await evaluate("document.querySelector('.audit-diagram-controls').open=true");await check('[name="diagram-audit-enabled"]');await check('[name="diagram-audit-questions"]');await field('[name="diagram-audit-question"]',safeguard);await field('[name="diagram-audit-filter-name"]','Safeguard diagram');await button('Save diagram audit filter');await waitFor("!document.body.innerText.includes('Working…')");await button('Fit');await sleep(600);
 await evaluate("document.querySelector('.canvas-host').scrollIntoView({block:'center'})");const geo=await evaluate("(()=>{const h=document.querySelector('.canvas-host'),r=h.getBoundingClientRect();return{x:r.x,y:r.y,vx:Number(h.dataset.viewX),vy:Number(h.dataset.viewY),scale:Number(h.dataset.viewScale)};})()");const position=model.tree.find(n=>n.id===barrier.id).properties['canvas.position'];
 const x=geo.x+geo.vx+(position.x+20)*geo.scale,y=geo.y+geo.vy+(position.y+72)*geo.scale;
 await send('Input.dispatchMouseEvent',{type:'mousePressed',x,y,button:'left',clickCount:1});await send('Input.dispatchMouseEvent',{type:'mouseReleased',x,y,button:'left',clickCount:1});await waitFor("document.body.innerText.includes('Audit answer details')");assert.ok(await evaluate("document.querySelector('[role=dialog]').innerText.includes('Local reviewer')"));await screenshot('diagram-answer-details.png');await button('Close audit details');await screenshot('audit-diagram.png');
 model=await reopenEqual(casePath);assert.equal(model.audits[0].savedFilters.length,2);
 const before=await invoke('case_get'),beforeBytes=await readFile(casePath);await button('Audits');const bad=resolve(folder,'invalid.xlsx');parts['xl/worksheets/sheet1.xml']=strToU8(sheet.replace(a.id,'wrong-audit'));
 // Invalid ordinal text is rejected before saving any new respondent record.
 parts['xl/worksheets/sheet1.xml']=strToU8(sheet.replace('Independent Excel-like response evidence','Independent Excel-like response evidence').replace(/(<c r="H2"[^>]*><is><t>).*?(<\/t>)/,'$1Unknown choice$2'));await writeFile(bad,zipSync(parts,{level:6,mtime:new Date(1980,0,1)}));await field('[name="audit-output-path"]',bad);await button('Import audit forms');await waitFor("!!document.querySelector('[role=alert]')");assert.deepEqual(await invoke('case_get'),before);assert.deepEqual(await readFile(casePath),beforeBytes);
 const exported=await invoke('case_table_export');const table=resolve(runRoot,'artifacts/audited-case.csv');await writeFile(table,exported.text);await invoke('case_create',{title:'Round-trip target',organization:'New organization'});const preview=await invoke('case_import_preview',{text:exported.text});assert.equal(preview.errors.length,0,JSON.stringify(preview.errors));await invoke('case_import_apply',{text:exported.text,signature:preview.signature});assert.equal((await invoke('case_table_export')).text,exported.text);await reopenEqual(resolve(folder,'roundtrip.sqlite'));
 const hashes={};for(const name of ['audit-form.xlsx','audit-filled.xlsx','audit-review.docx','audited-case.csv'])hashes[name]=createHash('sha256').update(await readFile(resolve(runRoot,'artifacts',name))).digest('hex');
 const result={result:'PASS',scope:'Windows native SPEC023 prepare/link/subset/conduct/review workflow against liquid transfer case',questionHierarchy:true,barrierAndManagementTargets:true,contextualAndSharedAnswers:true,completionAndRemarks:true,ordinalCounts:[3,0,1],mostNegativeChoice:2,excelLikeDateSerialImport:true,reviewReport:true,savedFilterReopen:true,linkedQuestionsOnShapes:true,graphClickDetailsDialog:true,allAuditSaveReopenEquality:true,rejectedWorkbookByteEquality:true,fullCaseTableAuditRoundTrip:true,hashes,manual:['Native chooser','Microsoft Excel actual application interoperability','macOS/Linux native execution']};
 await writeFile(resolve(runRoot,'gui-validation-results.json'),JSON.stringify(result,null,2)+'\n');await writeFile(resolve(runRoot,'audit-snapshot.json'),JSON.stringify(before.audits,null,2)+'\n');await writeFile(resolve(runRoot,'audit-results.json'),JSON.stringify(before.auditResults,null,2)+'\n');console.log(JSON.stringify(result,null,2));await invoke('case_close');
}finally{socket.close();}
