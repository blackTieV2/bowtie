param([switch]$ExportOnly)
$ErrorActionPreference = 'Stop'
$phase4Root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$phase4Port = 9335
if (Get-NetTCPConnection -LocalPort $phase4Port -State Listen -ErrorAction SilentlyContinue) { throw "Diagnostic port $phase4Port is occupied." }
$phase4Exe = Join-Path $phase4Root 'app/src-tauri/target/release/barrier-atlas.exe'
$phase4Directory = Get-Location
$phase4Variables = @('BARRIER_ATLAS_CDP_PORT','BARRIER_ATLAS_SMOKE_OUTPUT','BARRIER_ATLAS_RESULT_PATH','BARRIER_ATLAS_PHASE3_RUN','WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS','WEBVIEW2_USER_DATA_FOLDER')
$phase4Previous = @{}
foreach ($phase4Name in $phase4Variables) { $phase4Previous[$phase4Name] = [Environment]::GetEnvironmentVariable($phase4Name, 'Process') }
try {
 Set-Location -LiteralPath $phase4Root
 $env:BARRIER_ATLAS_CDP_PORT = "$phase4Port"
 $env:BARRIER_ATLAS_SMOKE_OUTPUT = Join-Path $phase4Root '.runtime/phase4-walkthrough'
 $env:BARRIER_ATLAS_RESULT_PATH = Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'phase2-regression.json'
 $env:BARRIER_ATLAS_PHASE3_RUN = Join-Path $phase4Root '.runtime/phase4-regression'
 $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$phase4Port"
 $env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $phase4Root '.runtime/phase4-webview'
 $phase4Process = Start-Process -FilePath $phase4Exe -WindowStyle Hidden -PassThru
 $phase4Exit = 0
 if (-not $ExportOnly) {
  node scripts/check_phase2_native.mjs
  $phase4Exit = $LASTEXITCODE
  if ($phase4Exit -eq 0) { node scripts/check_phase3_native.mjs; $phase4Exit = $LASTEXITCODE }
 }
 if ($phase4Exit -eq 0) { node scripts/check_phase4_native.mjs; $phase4Exit = $LASTEXITCODE }
} finally {
 if ($phase4Process -and -not $phase4Process.HasExited) { Stop-Process -Id $phase4Process.Id }
 foreach ($phase4Name in $phase4Variables) { [Environment]::SetEnvironmentVariable($phase4Name,$phase4Previous[$phase4Name],'Process') }
 Set-Location -LiteralPath $phase4Directory.Path
}
exit $phase4Exit
