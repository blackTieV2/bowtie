import type { CaseSnapshot } from "./model";
export type AuditQuestion={id:string;parentId:string|null;title:string;criteria:string;answers:string[];positiveFirst:boolean;mostNegative:boolean;contextual:boolean;targets:string[]};
export type AuditAnswer={questionId:string;contextId:string|null;choice:number|null;remark:string};
export type AuditFillout={id:string;date:string;respondent:string;unit:string;completed:boolean;answers:AuditAnswer[]};
export type AuditSurvey={id:string;title:string;code:string;description:string;questionIds:string[];fillouts:AuditFillout[]};
export type Audit={id:string;title:string;questions:AuditQuestion[];surveys:AuditSurvey[];savedFilters:AuditSavedFilter[]};
export type AuditSavedFilter={id:string;name:string;filter:AuditFilter;questionId:string|null;showQuestions:boolean};
export type AuditFilter={surveyId:string|null;unit:string|null;from:string|null;to:string|null;filloutId:string|null;targetId:string|null;negativeOnly:boolean};
export type AuditResult={questionId:string;title:string;choices:{label:string;tendency:string}[];counts:number[];unanswered:number;worstChoice:number|null;mostNegative:boolean;details:{surveyId:string;filloutId:string;respondent:string;date:string;unit:string;completed:boolean;contextId:string|null;choice:number|null;remark:string}[]};
export const emptyAuditFilter:AuditFilter={surveyId:null,unit:null,from:null,to:null,filloutId:null,targetId:null,negativeOnly:false};
export function auditChoices(q:AuditQuestion):string[]{return q.answers.filter(a=>a!=="@").map(a=>a.startsWith("@")?a.slice(1):a);}
export function auditSlots(a:Audit,s:AuditSurvey):AuditAnswer[]{return s.questionIds.flatMap<AuditAnswer>(id=>{const q=a.questions.find(q=>q.id===id)!;return q.contextual&&q.targets.length?q.targets.map(contextId=>({questionId:id,contextId,choice:null,remark:""})):[{questionId:id,contextId:null,choice:null,remark:""}];});}
export function entityPath(s:CaseSnapshot,id:string):string{const names:string[]=[];let next:string|null=id;while(next){const n=s.tree.find(n=>n.id===next);if(!n)break;names.push(n.title);next=n.parentId;}return names.reverse().join(" / ");}
export function questionPath(a:Audit,q:AuditQuestion):string{const names=[q.title];let next=q.parentId;while(next){const parent=a.questions.find(n=>n.id===next);if(!parent)break;names.push(parent.title);next=parent.parentId;}return names.reverse().join(" / ");}
export function relatedAuditTargets(s:CaseSnapshot,id:string):Set<string>{const set=new Set([id]);for(const l of s.links.filter(l=>l.sourceId===id&&["activity","document"].includes(l.relation))){set.add(l.targetId);if(l.relation==="activity")for(const d of s.links.filter(d=>d.sourceId===l.targetId&&d.relation==="document"))set.add(d.targetId);}return set;}
