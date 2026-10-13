"""Original application artifact QA; local PyMuPDF/Word are QA tools, not runtime dependencies."""
from pathlib import Path
import sys,json,zipfile,xml.etree.ElementTree as ET,csv,hashlib
sys.path.insert(0,str(Path('.runtime/phase4-python').resolve()))
import pymupdf as fitz
root=Path(sys.argv[1] if len(sys.argv)>1 else 'runs/008-build-phase7')
word_pdf=Path(sys.argv[2] if len(sys.argv)>2 else '.runtime/phase7-docx-render/incident-review-word.pdf')
render=root/'evidence/report-render';render.mkdir(parents=True,exist_ok=True)
doc=fitz.open(word_pdf)
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1,1)).save(render/f'page-{i+1}.png')
pdf=fitz.open(root/'artifacts/incident-diagram.pdf');pdf[0].get_pixmap(matrix=fitz.Matrix(1,1)).save(root/'evidence/incident-pdf-render.png')
with zipfile.ZipFile(root/'artifacts/incident-review.docx') as z:
 assert z.testzip() is None
 for name in z.namelist():
  if name.endswith('.xml') or name.endswith('.rels'):ET.fromstring(z.read(name))
 images=[name for name in z.namelist() if name.startswith('word/media/')];assert len(images)==2
 text=' '.join(ET.fromstring(z.read('word/document.xml')).itertext())
 assert 'Hose inspection at station' in text and 'First deviation' in text and '\ufffd' not in text
 assert 'actor Id:' not in text and 'column Id:' not in text
rows=list(csv.DictReader((root/'artifacts/incident-methods-case.csv').open(encoding='utf-8-sig',newline='')))
records=[json.loads(r['payload_json']) for r in rows if r['record_type']=='incident']
assert {r['methodKey'] for r in records}=={'systemic','bfa','trio','rca','categorized_rca'}
ET.parse(root/'artifacts/incident-diagram.svg')
report={'result':'PASS','docx_pages':len(doc),'docx_embedded_images':len(images),'diagram_pdf_pages':len(pdf),'methods_in_csv':len(records),'zip_xml_csv_json':'PASS','technical_UUID_labels_absent':True,'hashes':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((root/'artifacts').iterdir())}}
(root/'producer-artifact-checks.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
print(json.dumps(report,indent=2))
