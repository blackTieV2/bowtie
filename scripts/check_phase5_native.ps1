param([switch]$ToolsOnly, [string]$EvidenceRoot='runs/006-build-phase5', [string]$RuntimeRoot='.runtime/phase5-walkthrough')
$ErrorActionPreference = 'Stop'
$phase5Root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$phase5Port = 9336
if (Get-NetTCPConnection -LocalPort $phase5Port -State Listen -ErrorAction SilentlyContinue) { throw "Diagnostic port $phase5Port is occupied." }
$phase5Directory = Get-Location
$phase5Variables = @('BARRIER_ATLAS_CDP_PORT','BARRIER_ATLAS_SMOKE_OUTPUT','BARRIER_ATLAS_RESULT_PATH','BARRIER_ATLAS_PHASE3_RUN','BARRIER_ATLAS_PHASE5_RUN','WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS','WEBVIEW2_USER_DATA_FOLDER')
$phase5Previous = @{}
foreach ($phase5Name in $phase5Variables) { $phase5Previous[$phase5Name] = [Environment]::GetEnvironmentVariable($phase5Name, 'Process') }
try {
 Set-Location -LiteralPath $phase5Root
 $env:BARRIER_ATLAS_CDP_PORT = "$phase5Port"
 $env:BARRIER_ATLAS_SMOKE_OUTPUT = Join-Path $phase5Root $RuntimeRoot
 $env:BARRIER_ATLAS_RESULT_PATH = Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'phase2-regression.json'
 $env:BARRIER_ATLAS_PHASE3_RUN = Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'regression'
 $env:BARRIER_ATLAS_PHASE5_RUN = Join-Path $phase5Root $EvidenceRoot
 $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$phase5Port"
 $env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'webview'
 $phase5Process = Start-Process -FilePath (Join-Path $phase5Root 'app/src-tauri/target/release/barrier-atlas.exe') -WindowStyle Hidden -PassThru
 $phase5Exit=0
 if (-not $ToolsOnly) { node scripts/check_phase2_native.mjs; $phase5Exit=$LASTEXITCODE; if ($phase5Exit -eq 0) { node scripts/check_phase3_native.mjs; $phase5Exit=$LASTEXITCODE } }
 if ($phase5Exit -eq 0) { node scripts/check_phase5_compare_native.mjs; $phase5Exit=$LASTEXITCODE }
 if ($phase5Exit -eq 0) { node scripts/check_phase5_native.mjs; $phase5Exit=$LASTEXITCODE }
} finally {
 if ($phase5Process -and -not $phase5Process.HasExited) { Stop-Process -Id $phase5Process.Id }
 foreach ($phase5Name in $phase5Variables) { [Environment]::SetEnvironmentVariable($phase5Name,$phase5Previous[$phase5Name],'Process') }
 Set-Location -LiteralPath $phase5Directory.Path
}
exit $phase5Exit
