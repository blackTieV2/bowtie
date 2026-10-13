# Visual QA only: render this app's generated DOCX through an owned Word instance.
$ErrorActionPreference = 'Stop'
$phase4Root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$phase4Docx = Join-Path $phase4Root 'runs/005-build-phase4/artifacts/run-1/case-report.docx'
$phase4Pdf = Join-Path $phase4Root '.runtime/phase4-docx-render/case-report-word.pdf'
New-Item -ItemType Directory -Path (Split-Path -Parent $phase4Pdf) -Force | Out-Null
try {
 $phase4Word = New-Object -ComObject Word.Application
 $phase4Word.Visible = $false
 $phase4Word.DisplayAlerts = 0
 $phase4Document = $phase4Word.Documents.Open($phase4Docx,$false,$true)
 $phase4Document.ExportAsFixedFormat($phase4Pdf,17)
 Write-Output "Rendered generated report through Microsoft Word: $phase4Pdf"
} finally {
 if ($phase4Document) { $phase4Document.Close(0); [void][Runtime.InteropServices.Marshal]::ReleaseComObject($phase4Document) }
 if ($phase4Word) { $phase4Word.Quit(); [void][Runtime.InteropServices.Marshal]::ReleaseComObject($phase4Word) }
}
