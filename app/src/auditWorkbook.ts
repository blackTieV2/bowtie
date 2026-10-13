import {strFromU8,strToU8,unzipSync,zipSync} from "fflate";
import {csvRows,fromWorkbook,toWorkbook} from "./tableWorkbook";
import {xml} from "./exportXml";
import type {CaseSnapshot} from "./model";
import {auditChoices,auditSlots,entityPath,type Audit,type AuditSurvey,type AuditFillout} from "./auditModel";
export const auditHeader=["survey_id","question_id","context_id","question","criteria","context","answer_options","answer","date","respondent","unit","remark"];
const csv=(rows:string[][])=>rows.map(r=>r.map(c=>`"${c.replaceAll('"','""')}"`).join(",")).join("\r\n")+"\r\n";
const column=(i:number):string=>i<26?String.fromCharCode(65+i):column(Math.floor(i/26)-1)+String.fromCharCode(65+i%26);
export function auditFormWorkbook(snapshot:CaseSnapshot,audit:Audit,survey:AuditSurvey):Uint8Array{
 const slots=auditSlots(audit,survey);
 const rows=[auditHeader,...slots.map(slot=>{const q=audit.questions.find(q=>q.id===slot.questionId)!;return [survey.id,q.id,slot.contextId??"",q.title,q.criteria,slot.contextId?entityPath(snapshot,slot.contextId):"Shared / general",JSON.stringify(auditChoices(q)),"","","","",""];})];
 const files=unzipSync(toWorkbook(csv(rows),"Audit form"));
 const questions=survey.questionIds.map(id=>audit.questions.find(q=>q.id===id)!);
 if(questions.length>16384)throw Error("The survey exceeds the workbook column limit; split it into smaller surveys.");
 const ns="http://schemas.openxmlformats.org/spreadsheetml/2006/main";
 const optionRows=Array.from({length:Math.max(1,...questions.map(q=>auditChoices(q).length))},(_,r)=>`<row r="${r+1}">${questions.map((q,c)=>{const label=auditChoices(q)[r];return label===undefined?"":`<c r="${column(c)}${r+1}" t="inlineStr"><is><t xml:space="preserve">${xml(label)}</t></is></c>`;}).join("")}</row>`).join("");
 files["xl/worksheets/sheet2.xml"]=strToU8(`<?xml version="1.0"?><worksheet xmlns="${ns}"><sheetData>${optionRows}</sheetData></worksheet>`);
 const names=questions.map((q,c)=>`<definedName name="AuditChoices_${c}">Choices!$${column(c)}$1:$${column(c)}$${auditChoices(q).length}</definedName>`).join("");
 files["xl/workbook.xml"]=strToU8(strFromU8(files["xl/workbook.xml"]).replace('</sheets>','<sheet name="Choices" sheetId="2" state="hidden" r:id="rId2"/></sheets>').replace('</workbook>',`<definedNames>${names}</definedNames></workbook>`));
 files["xl/_rels/workbook.xml.rels"]=strToU8(strFromU8(files["xl/_rels/workbook.xml.rels"]).replace('</Relationships>','<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/></Relationships>'));
 files["xl/styles.xml"]=strToU8('<?xml version="1.0"?><styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><fonts count="1"><font><sz val="11"/><name val="Calibri"/></font></fonts><fills count="2"><fill><patternFill patternType="none"/></fill><fill><patternFill patternType="gray125"/></fill></fills><borders count="1"><border/></borders><cellStyleXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0"/></cellStyleXfs><cellXfs count="2"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/><xf numFmtId="49" fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/></cellXfs></styleSheet>');
 files["xl/_rels/workbook.xml.rels"]=strToU8(strFromU8(files["xl/_rels/workbook.xml.rels"]).replace('</Relationships>','<Relationship Id="rIdStyles" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/></Relationships>'));
 files["[Content_Types].xml"]=strToU8(strFromU8(files["[Content_Types].xml"]).replace('</Types>','<Override PartName="/xl/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"/></Types>'));
 files["xl/worksheets/sheet1.xml"]=strToU8(strFromU8(files["xl/worksheets/sheet1.xml"]).replace(/<c r="([HI]\d+)"/g,'<c r="$1" s="1"'));
 files["[Content_Types].xml"]=strToU8(strFromU8(files["[Content_Types].xml"]).replace('</Types>','<Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>'));
 const validations=slots.map((slot,i)=>`<dataValidation type="list" allowBlank="1" showErrorMessage="1" errorTitle="Choose a listed answer" error="Select an answer from the question scale" sqref="H${i+2}"><formula1>AuditChoices_${survey.questionIds.indexOf(slot.questionId)}</formula1></dataValidation>`).join("");
 files["xl/worksheets/sheet1.xml"]=strToU8(strFromU8(files["xl/worksheets/sheet1.xml"]).replace('</worksheet>',`<dataValidations count="${slots.length}">${validations}</dataValidations></worksheet>`));
 return zipSync(files,{level:6,mtime:new Date(1980,0,1)});
}
export function readAuditForm(bytes:Uint8Array,audit:Audit,survey:AuditSurvey):AuditFillout{
 const sheets=fromWorkbook(bytes,12,true);if(!sheets.length)throw Error("The workbook contains no audit form");
 const rows=csvRows(sheets[0].text);if(JSON.stringify(rows[0])!==JSON.stringify(auditHeader))throw Error("Expected the original audit fill-out columns");
 // Spreadsheet programs may turn a pasted date into a styled serial despite the
 // template's text format. Convert only actual calendar-date cells, not arbitrary numbers.
 const parts=unzipSync(bytes);
 const doc=(name:string)=>new DOMParser().parseFromString(strFromU8(parts[name]),'application/xml');
 const elements=(d:Document|Element,name:string)=>[...d.getElementsByTagNameNS('*',name)];
 const workbook=doc('xl/workbook.xml');const first=elements(workbook,'sheet')[0];const relation=elements(doc('xl/_rels/workbook.xml.rels'),'Relationship').find(r=>r.getAttribute('Id')===first.getAttributeNS('http://schemas.openxmlformats.org/officeDocument/2006/relationships','id'))!;
 const target=relation.getAttribute('Target')!;const sheetDoc=doc(target.startsWith('/')?target.slice(1):`xl/${target}`);
 const date1904=['1','true'].includes(elements(workbook,'workbookPr')[0]?.getAttribute('date1904')??'');
 const styles=parts['xl/styles.xml']?doc('xl/styles.xml'):null;
 const custom=new Map(styles?elements(styles,'numFmt').map(e=>[Number(e.getAttribute('numFmtId')),e.getAttribute('formatCode')??'']):[]);
 const xfs=styles?elements(styles,'cellXfs')[0]:undefined;const formats=xfs?elements(xfs,'xf').map(e=>Number(e.getAttribute('numFmtId'))):[];
 for(const cell of elements(sheetDoc,'c')){
  const reference=cell.getAttribute('r')??'';if(!/^I[2-9]\d*$|^I1\d+$/.test(reference))continue;
  const index=Number(reference.slice(1))-1;if(!rows[index])continue;
  const type=cell.getAttribute('t');const value=elements(cell,'v')[0]?.textContent??'';
  if(type==='d'){rows[index][8]=value.slice(0,10);continue;}
  if(type&&type!=='n')continue;
  const format=formats[Number(cell.getAttribute('s')??0)]??0;
  const code=(custom.get(format)??'').replace(/"[^\"]*"|\\.|\[[^\]]*\]/g,'');
  if(![14,15,16,17,22].includes(format)&&!(/[yd]/i.test(code)))continue;
  const serial=Number(value);if(!value||!Number.isFinite(serial)||serial<0||(!date1904&&Math.floor(serial)===60))throw Error(`Row ${index+1}: invalid spreadsheet calendar date`);
  const days=Math.floor(serial);const epoch=Date.UTC(date1904?1904:1899,date1904?0:11,date1904?1:31);const date=new Date(epoch+(date1904?days:days>=60?days-1:days)*86400000);
  const iso=date.toISOString().slice(0,10);if(!/^\d{4}-\d{2}-\d{2}$/.test(iso))throw Error(`Row ${index+1}: date outside the supported range`);rows[index][8]=iso;
 }
 const slots=auditSlots(audit,survey);if(rows.length!==slots.length+1)throw Error("The form must contain every question/context exactly once");
 const answers:AuditFillout["answers"]=[];const seen=new Set<string>();let metadata:string[]|undefined;
 for(const [i,row] of rows.slice(1).entries()){
  const line=i+2;if(row.length!==12||row[0]!==survey.id)throw Error(`Row ${line}: wrong survey or column count`);
  const q=audit.questions.find(q=>q.id===row[1]);const context=row[2]||null;
  if(!q||!slots.some(s=>s.questionId===q.id&&s.contextId===context))throw Error(`Row ${line}: unknown question/context`);
  const key=JSON.stringify([q.id,context]);if(seen.has(key))throw Error(`Row ${line}: duplicate response`);seen.add(key);
  if(row[3]!==q.title||row[4]!==q.criteria||row[6]!==JSON.stringify(auditChoices(q)))throw Error(`Row ${line}: question/scale changed after export; regenerate the form`);
  const current=row.slice(8,11);if(metadata&&JSON.stringify(metadata)!==JSON.stringify(current))throw Error(`Row ${line}: date/respondent/unit must match throughout the form`);metadata=current;
  const choice=row[7]?auditChoices(q).indexOf(row[7]):null;if(choice===-1)throw Error(`Row ${line}: unknown ordinal answer`);
  answers.push({questionId:q.id,contextId:context,choice,remark:row[11]});
 }
 if(!metadata)throw Error("Select questions before exporting a fill-out form");
 return {id:crypto.randomUUID(),date:metadata[0],respondent:metadata[1],unit:metadata[2],completed:answers.every(r=>r.choice!==null),answers};
}
