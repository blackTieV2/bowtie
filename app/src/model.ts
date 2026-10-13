export type Point = { x: number; y: number };
export type TreeRow = { id: string; parentId: string | null; kind: string; title: string; sortOrder: number; depth: number; description: string; properties: Record<string, unknown>; effectivenessId: string | null };
export type LinkRow = { sourceId: string; targetId: string; relation: string };
export type Axis = {code:string;title:string};
export type Category = {id:string;title:string;color:string;rank:number};
export type Cell = {likelihoodPosition:number;severityPosition:number;categoryId:string;label:string};
export type Matrix = {id:string;title:string;coordinateOrder:"xy"|"yx";likelihood:Axis[];severity:Axis[];categories:Category[];cells:Cell[]};
export type Assessment = {targetId:string;matrixId:string;phase:"inherent"|"residual";likelihoodPosition:number;severityPosition:number};
export type Lookup = {id:string;tableKey:string;title:string;code:string;color:string;sortOrder:number;metadata:Record<string,unknown>};
export type ActionRow = {id:string;parentId:string;parentTitle:string;title:string;description:string;properties:Record<string,unknown>;responsible:string[]};
export type CaseSnapshot = { title: string; path: string | null; dirty: boolean; tree: TreeRow[]; links: LinkRow[]; matrices:Matrix[];assessments:Assessment[];lookups:Lookup[];actions:ActionRow[];customFields:Record<string,Record<string,string>>;audits:import("./auditModel").Audit[];auditResults:Record<string,import("./auditModel").AuditResult[]> };
export const label = (kind: string) => kind.replaceAll("_", " ");
export const box = { width: 170, height: 78 };
export function diagramFor(rows: TreeRow[], selected?: TreeRow): TreeRow | undefined {
  let node = selected;
  while (node) {
    if (node.kind === "diagram") return node;
    if (node.kind === "hazard") return rows.find(row => row.parentId === node?.id && row.kind === "diagram");
    node = rows.find(row => row.id === node?.parentId);
  }
}
export function diagramNodes(rows: TreeRow[], diagram: TreeRow): TreeRow[] {
  const ids = new Set([diagram.id]);
  for (const row of rows) if (row.parentId && ids.has(row.parentId)) ids.add(row.id);
  return rows.filter(row => (ids.has(row.id) && !["diagram", "action"].includes(row.kind)) || row.id === diagram.parentId);
}
// Coordinates are box origins in diagram units. Layout is deterministic and
// branch spacing reserves space for all nested escalation branches.
export function classicLayout(nodes: TreeRow[]): Record<string, Point> {
  const result: Record<string, Point> = {};
  const children = (id: string, kind: string) => nodes.filter(n => n.parentId === id && n.kind === kind).sort((a,b) => a.sortOrder-b.sortOrder || a.id.localeCompare(b.id));
  const top = nodes.find(n => n.kind === "top_event");
  if (!top) return result;
  const branchHeight = (id: string): number => {
    const factors = children(id, "barrier").flatMap(b => children(b.id, "escalation_factor"));
    return 130 + factors.reduce((sum,f) => sum + branchHeight(f.id), 0);
  };
  const threats = children(top.id,"threat"), consequences = children(top.id,"consequence");
  const count = Math.max(1,...nodes.filter(n => ["threat","consequence","escalation_factor"].includes(n.kind)).map(n => children(n.id,"barrier").length));
  const reach = Math.max(680, (count + 1) * 220);
  const center = reach + 80;
  const total = Math.max(threats.reduce((s,n) => s+branchHeight(n.id),0), consequences.reduce((s,n) => s+branchHeight(n.id),0),130);
  result[top.id] = { x:center, y:160 + total/2 - box.height/2 };
  const hazard = nodes.find(n => n.kind === "hazard");
  if (hazard) result[hazard.id] = {x:center,y:35};
  function branch(node: TreeRow, side: -1 | 1, y: number, outer: number, inner: number) {
    result[node.id] = {x:outer,y};
    const barriers = children(node.id,"barrier");
    const pathFrom = side === -1 ? outer+box.width : inner+box.width;
    const pathTo = side === -1 ? inner : outer;
    barriers.forEach((b,i) => { result[b.id] = {x:pathFrom+(pathTo-pathFrom)*(side === -1 ? (i+1)/(barriers.length+1) : (barriers.length-i)/(barriers.length+1))-box.width/2,y}; });
    let factorY = y+130;
    for (const barrier of barriers) for (const factor of children(barrier.id,"escalation_factor")) {
      const anchor = result[barrier.id].x;
      const factorReach = (children(factor.id,"barrier").length+1)*210;
      branch(factor,side,factorY,anchor+side*factorReach,anchor);
      factorY += branchHeight(factor.id);
    }
  }
  for (const [side,paths] of [[-1,threats],[1,consequences]] as const) {
    let y=160;
    for (const path of paths) { branch(path,side,y,center+side*reach,center); y+=branchHeight(path.id); }
  }
  return result;
}
export function savedPositions(nodes: TreeRow[]): Record<string, Point> {
  const result = classicLayout(nodes);
  for (const node of nodes) {
    const p = node.properties["canvas.position"] as Point | undefined;
    if (p && Number.isFinite(p.x) && Number.isFinite(p.y)) result[node.id] = p;
  }
  return result;
}
