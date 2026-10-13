import { strFromU8, strToU8, unzipSync, zipSync } from "fflate";
const MAX = 20 * 1024 * 1024;
export function csvRows(text: string): string[][] {
  const rows: string[][] = [], row: string[] = []; let cell = "", quoted = false, closed = false;
  text = text.replace(/^\uFEFF/, "");
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quoted) { if (c === '"') { if (text[i + 1] === '"') { cell += '"'; i++; } else { quoted = false; closed = true; } } else cell += c; continue; }
    if (c === '"' && !cell && !closed) quoted = true;
    else if (c === ",") { row.push(cell); cell = ""; closed = false; }
    else if (c === "\r" || c === "\n") { if (c === "\r" && text[i + 1] === "\n") i++; row.push(cell); rows.push([...row]); row.length = 0; cell = ""; closed = false; }
    else if (c === '"' || closed) throw Error("Malformed CSV quoting");
    else cell += c;
  }
  if (quoted) throw Error("Unclosed CSV quote");
  if (cell || row.length || closed) { row.push(cell); rows.push([...row]); }
  return rows;
}
const encode = (rows: string[][]) => rows.map(row => row.map(c => `"${c.replaceAll('"', '""')}"`).join(",")).join("\r\n") + "\r\n";
function xml(text: string): string {
  if (/[\x00-\x08\x0B\x0C\x0E-\x1F]/.test(text)) throw Error("Spreadsheet text contains an unsupported control character");
  return text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;").replaceAll("'", "&apos;").replaceAll("\r", "&#13;");
}
const ns = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
export function toWorkbook(text: string): Uint8Array {
  const rows = csvRows(text);
  if (rows.some(row => row.some(cell => cell.length > 32767))) throw Error("A spreadsheet cell exceeds 32,767 characters; export this case as CSV.");
  const sheet = `<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="${ns}"><sheetData>${rows.map((row, r) => `<row r="${r + 1}">${row.map((cell, c) => `<c r="${String.fromCharCode(65 + c)}${r + 1}" t="inlineStr"><is><t xml:space="preserve">${xml(cell)}</t></is></c>`).join("")}</row>`).join("")}</sheetData></worksheet>`;
  const parts = {
    "[Content_Types].xml": '<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>',
    "_rels/.rels": '<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>',
    "xl/workbook.xml": `<?xml version="1.0"?><workbook xmlns="${ns}" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Case data" sheetId="1" r:id="rId1"/></sheets></workbook>`,
    "xl/_rels/workbook.xml.rels": '<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>',
    "xl/worksheets/sheet1.xml": sheet,
  };
  return zipSync(Object.fromEntries(Object.entries(parts).map(([name, value]) => [name, strToU8(value)])), { level: 6, mtime: new Date(1980, 0, 1) });
}
export function fromWorkbook(bytes: Uint8Array): { name: string; text: string }[] {
  let total = 0;
  const parts = unzipSync(bytes, { filter: file => { total += file.originalSize; if (total > MAX || file.originalSize > MAX) throw Error("Expanded spreadsheet exceeds 20 MiB"); return file.name.endsWith(".xml") || file.name.endsWith(".rels"); } });
  function document(name: string): Document {
    if (!parts[name]) throw Error(`Missing spreadsheet part: ${name}`);
    const text = strFromU8(parts[name]); if (/<!DOCTYPE|<!ENTITY/i.test(text)) throw Error("Spreadsheet XML declarations are unsupported");
    const doc = new DOMParser().parseFromString(text, "application/xml"); if (doc.getElementsByTagName("parsererror").length) throw Error(`Malformed spreadsheet XML: ${name}`); return doc;
  }
  const elements = (e: Document | Element, name: string) => [...e.getElementsByTagNameNS("*", name)];
  const textOf = (e: Element, name: string) => elements(e, name).map(t => t.textContent ?? "").join("");
  const shared = parts["xl/sharedStrings.xml"] ? elements(document("xl/sharedStrings.xml"), "si").map(e => textOf(e, "t")) : [];
  const relationships = elements(document("xl/_rels/workbook.xml.rels"), "Relationship");
  return elements(document("xl/workbook.xml"), "sheet").map(sheet => {
    const id = sheet.getAttributeNS("http://schemas.openxmlformats.org/officeDocument/2006/relationships", "id");
    const relation = relationships.find(r => r.getAttribute("Id") === id);
    const target = relation?.getAttribute("Target");
    if (!target || target.includes("..") || relation?.getAttribute("TargetMode") === "External") throw Error("Unsupported worksheet relationship");
    const name = target.startsWith("/") ? target.slice(1) : `xl/${target}`;
    const rows: string[][] = []; let expectedRow = 1;
    for (const row of elements(document(name), "row")) {
      const number = Number(row.getAttribute("r") ?? expectedRow);
      if (!Number.isInteger(number) || number < expectedRow || number > 100_000) throw Error("Invalid spreadsheet row number");
      while (expectedRow < number) { rows.push(Array(8).fill("")); expectedRow++; }
      const cells: string[] = Array(8).fill(""); const seen = new Set<number>();
      for (const cell of elements(row, "c")) {
        const reference = cell.getAttribute("r") ?? "";
        const match = /^([A-H])([1-9]\d*)$/.exec(reference);
        if (!match || Number(match[2]) !== number || seen.has(match[1].charCodeAt(0))) throw Error(`Row ${number}: expected unique cells in columns A–H`);
        if (elements(cell, "f").length) throw Error(`Row ${number}: formulas are unsupported; paste values first`);
        seen.add(match[1].charCodeAt(0));
        const type = cell.getAttribute("t"), value = textOf(cell, "v");
        if (type === "s" && (!/^\d+$/.test(value) || !shared[Number(value)] && shared[Number(value)] !== "")) throw Error(`Row ${number}: invalid shared string`);
        cells[match[1].charCodeAt(0) - 65] = type === "inlineStr" ? textOf(cell, "t") : type === "s" ? shared[Number(value)] : value;
      }
      rows.push(cells); expectedRow++;
    }
    return { name: sheet.getAttribute("name") ?? "Worksheet", text: encode(rows) };
  });
}
