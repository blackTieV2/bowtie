"""Mechanical checks independent of the app's format builders."""
from pathlib import Path
from zipfile import ZipFile
import xml.etree.ElementTree as ET
import hashlib,json
from PIL import Image
from pypdf import PdfReader
root=Path('runs/005-build-phase4')
result={'result':'PASS','scope':'independent format parsers; producer-run validation','checks':{},'hashes':{}}
for run in ['run-1','run-2']:
    folder=root/'artifacts'/run
    svg=ET.parse(folder/'diagram.svg').getroot()
    assert svg.tag=='{http://www.w3.org/2000/svg}svg'
    assert svg.findall('.//{http://www.w3.org/2000/svg}text')
    assert not svg.findall('.//{http://www.w3.org/2000/svg}image'), 'SVG must be vector'
    with Image.open(folder/'diagram.png') as png:
        png.verify()
    with Image.open(folder/'diagram.png') as png:
        assert png.width==int(svg.attrib['width'])*4 and png.height==int(svg.attrib['height'])*4
        dimensions=list(png.size)
    pdf=PdfReader(folder/'diagram.pdf');assert len(pdf.pages)==1
    text=pdf.pages[0].extract_text();assert 'Loss of containment' in text and 'Hose rupture' in text
    assert not pdf.metadata.get('/CreationDate') and not pdf.metadata.get('/ModDate')
    with ZipFile(folder/'case-report.docx') as z:
        assert z.testzip() is None
        for name in z.namelist():
            if name.endswith('.xml') or name.endswith('.rels'): ET.fromstring(z.read(name))
        doc=ET.fromstring(z.read('word/document.xml'))
        words=' '.join(doc.itertext())
        for expected in ['Transfer safety review','Transfer station','Loss of containment','Safety assessment','Review isolation response','Effectiveness definitions','Verified operational effectiveness','Effective','Transfer operator']:
            assert expected in words,expected
        assert all(i.date_time==(1980,1,1,0,0,0) for i in z.infolist())
    result['checks'][run]={'pngDimensions':dimensions,'svgVector':True,'pdfPages':len(pdf.pages),'pdfTextExtractable':True,'docxXmlValid':True,'knownCaseStrings':True,'noCreationDates':True}
    result['hashes'][run]={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(folder.iterdir())}
assert result['hashes']['run-1']==result['hashes']['run-2']
result['byteIdentical']=True
(root/'artifact-validation-results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
