# QA only; run with PowerShell 7 (pwsh), using a task-owned hidden Word instance.
param([string]$Document='runs/008-build-phase7/artifacts/incident-review.docx',[string]$Output='.runtime/phase7-docx-render/incident-review-word.pdf')
$ErrorActionPreference='Stop'
$incidentRoot=(Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$incidentDocumentPath=(Resolve-Path (Join-Path $incidentRoot $Document)).Path
$incidentOutputPath=Join-Path $incidentRoot $Output
New-Item -ItemType Directory -Force (Split-Path -Parent $incidentOutputPath) | Out-Null
try {
 $incidentWord=New-Object -ComObject Word.Application
 $incidentWord.Visible=$false
 $incidentWord.DisplayAlerts=0
 $incidentDocument=$incidentWord.Documents.Open($incidentDocumentPath,$false,$true)
 $incidentDocument.ExportAsFixedFormat($incidentOutputPath,17)
 Write-Output $incidentOutputPath
}finally{
 if($incidentDocument){$incidentDocument.Close(0);[void][Runtime.InteropServices.Marshal]::ReleaseComObject($incidentDocument)}
 if($incidentWord){$incidentWord.Quit();[void][Runtime.InteropServices.Marshal]::ReleaseComObject($incidentWord)}
}
