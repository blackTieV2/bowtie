import {useState} from 'react';
import {invoke} from '@tauri-apps/api/core';
import {save as saveDialog} from '@tauri-apps/plugin-dialog';
import {type CaseSnapshot,type TreeRow} from './model';
type Context={snapshot:CaseSnapshot;signature:string};
export default function ExportDialog({diagram,busy,error,run,close}:{diagram?:TreeRow;busy:boolean;error:string;run:(a:()=>Promise<void>)=>Promise<boolean>;close:()=>void}){
 const [format,setFormat]=useState(diagram?'png':'docx'),[scale,setScale]=useState(4),[path,setPath]=useState(''),[result,setResult]=useState('');
 async function browse(){try{const p=await saveDialog({title:'Export',defaultPath:`${format==='docx'?'case-report':'diagram'}.${format}`,filters:[{name:format.toUpperCase(),extensions:[format]}]});if(p)setPath(p);}catch(e){setResult(String(e));}}
 async function exportFile(){await run(async()=>{const context=await invoke<Context>('case_export_context');let bytes:Uint8Array;
 if(format==='docx'){const {reportDocx}=await import('./caseReport');bytes=reportDocx(context.snapshot);}
 else {const d=context.snapshot.tree.find(n=>n.id===diagram?.id&&n.kind==='diagram');if(!d)throw Error('Select a diagram before exporting');const {diagramExport}=await import('./diagramExports');bytes=await diagramExport(context.snapshot,d,format,scale);}
 await invoke('case_write_export',{path,format,bytes:Array.from(bytes),signature:context.signature});setResult(`Exported ${format.toUpperCase()} to ${path}`);
 });}
 return <div className="overlay"><section className="modal" role="dialog" aria-modal="true"><h2>Export</h2><label>Format<select name="export-format" disabled={busy} value={format} onChange={e=>{setFormat(e.target.value);setPath('');setResult('');}}>{(diagram?['png','svg','pdf','docx']:['docx']).map(f=><option key={f} value={f}>{f==='docx'?'Word compatible case report':f.toUpperCase()+' diagram'}</option>)}</select></label>{format==='png'&&<label>PNG size multiplier<select name="export-scale" disabled={busy} value={scale} onChange={e=>setScale(Number(e.target.value))}>{[1,2,3,4].map(n=><option key={n} value={n}>{n}×</option>)}</select></label>}<label>Destination<input name="export-path" disabled={busy} value={path} onChange={e=>setPath(e.target.value)} placeholder={`Choose a destination ending in .${format}`}/></label><button disabled={busy} onClick={()=>void browse()}>Browse</button>{format!=='docx'&&<p className="muted">Exports include the full diagram at its current saved positions. Zoom, pan and selection highlights do not change the output.</p>}{error&&<p className="error" role="alert">{error}</p>}{result&&<p role="status">{result}</p>}<div className="actions"><button disabled={busy} onClick={close}>Close export</button><button disabled={busy||!path} onClick={()=>void exportFile()}>Export file</button></div></section></div>;
}
