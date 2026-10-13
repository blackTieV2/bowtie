"""Independent standard-library parsing of original Phase 5 table artifacts."""
import csv
import hashlib
import io
import json
from pathlib import Path
import sys
import xml.etree.ElementTree as ET
import zipfile

run = Path(sys.argv[1] if len(sys.argv) > 1 else "runs/006-build-phase5")
artifacts = run / "artifacts"
csv_bytes = (artifacts / "case-data.csv").read_bytes()
assert csv_bytes == (artifacts / "case-data-reexport.csv").read_bytes()
rows = list(csv.reader(io.StringIO(csv_bytes.decode("utf-8"), newline="")))
assert rows[0] == ["record_type", "record_id", "parent_id", "kind", "title", "description", "sort_order", "payload_json"]
assert all(len(row) == 8 for row in rows)
types = {}
ids = set()
for row in rows[1:]:
    types[row[0]] = types.get(row[0], 0) + 1
    assert (row[0], row[1]) not in ids
    ids.add((row[0], row[1]))
    assert isinstance(json.loads(row[7]), dict)
entities = {r[1]: r for r in rows[1:] if r[0] == "entity"}
assert entities["@root"][3] == "organization"
assert all(r[2] in entities for r in entities.values() if r[1] != "@root")
assert any(r[3] == "action" for r in entities.values())
assert any("canvas.position" in json.loads(r[7])["properties"] for r in entities.values())
assert types["matrix"] >= 1 and types["link"] >= 1 and types["assessment"] >= 1
ns = {"s": "http://schemas.openxmlformats.org/spreadsheetml/2006/main"}
with zipfile.ZipFile(artifacts / "case-data.xlsx") as archive:
    assert archive.testzip() is None
    for name in archive.namelist():
        if name.endswith(".xml") or name.endswith(".rels"):
            ET.fromstring(archive.read(name))
    doc = ET.fromstring(archive.read("xl/worksheets/sheet1.xml"))
    assert not doc.findall(".//s:f", ns)
    spreadsheet_rows = []
    for row in doc.findall("s:sheetData/s:row", ns):
        cells = [""] * 8
        for cell in row.findall("s:c", ns):
            assert cell.attrib["t"] == "inlineStr"
            index = ord(cell.attrib["r"][0]) - ord("A")
            cells[index] = "".join(t.text or "" for t in cell.findall(".//s:t", ns))
        spreadsheet_rows.append(cells)
    assert spreadsheet_rows == rows, "Independent XML/CSV structural equality"
diff = json.loads((artifacts / "case-differences.txt").read_text(encoding="utf-8"))
assert diff["differences"]
assert any(d["table"] == "entity" and d["field"] == "title" and "Scheduled" in d["before"] and "Pre-transfer" in d["after"] for d in diff["differences"])
result = {"result": "PASS", "parser": "Python stdlib csv, zipfile, ElementTree, json; independent of application serializers", "recordCounts": types, "entities": len(entities), "csvReexportByteEquality": True, "xlsxCsvStructuralEquality": True, "diffReportKnownChange": True, "hashes": {name: hashlib.sha256((artifacts / name).read_bytes()).hexdigest() for name in ["case-data.csv", "case-data-reexport.csv", "case-data.xlsx", "case-differences.txt"]}}
(run / "artifact-validation-results.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
print(json.dumps(result, indent=2))
