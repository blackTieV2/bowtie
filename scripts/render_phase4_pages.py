"""Local visual QA. PyMuPDF is a QA dependency, never an app dependency."""
from pathlib import Path
import sys,json
sys.path.insert(0,str(Path('.runtime/phase4-python').resolve()))
import pymupdf as fitz
root=Path('runs/005-build-phase4/evidence/final-render')
root.mkdir(parents=True,exist_ok=True)
counts={}
for name,path in [('diagram',Path('runs/005-build-phase4/artifacts/run-1/diagram.pdf')),('report',Path('.runtime/phase4-docx-render/case-report-word.pdf'))]:
    doc=fitz.open(path)
    counts[name]=len(doc)
    for index,page in enumerate(doc):
        scale=min(1.4,1600/page.rect.width)
        page.get_pixmap(matrix=fitz.Matrix(scale,scale)).save(root/f'{name}-page-{index+1}.png')
(root.parent/'render-results.json').write_text(json.dumps({'renderer':'Microsoft Word DOCX-to-PDF; PyMuPDF page rasterization','pages':counts,'packagedRenderer':'Unavailable: LibreOffice soffice.exe absent; native Word fallback used'},indent=2)+'\n')
print(counts)
