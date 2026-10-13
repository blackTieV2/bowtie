$ErrorActionPreference = 'Stop'
$phase3Root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$phase3Port = 9334
if (Get-NetTCPConnection -LocalPort $phase3Port -State Listen -ErrorAction SilentlyContinue) { throw "Diagnostic port $phase3Port is occupied." }
$phase3Exe = Join-Path $phase3Root 'app/src-tauri/target/release/barrier-atlas.exe'
if (-not (Test-Path -LiteralPath $phase3Exe)) { throw 'Build the application first.' }
$phase3Directory = Get-Location
$phase3Variables = @('BARRIER_ATLAS_CDP_PORT','BARRIER_ATLAS_SMOKE_OUTPUT','BARRIER_ATLAS_RESULT_PATH','WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS','WEBVIEW2_USER_DATA_FOLDER')
$phase3Previous = @{}
foreach ($phase3Name in $phase3Variables) { $phase3Previous[$phase3Name] = [Environment]::GetEnvironmentVariable($phase3Name, 'Process') }
try {
 Set-Location -LiteralPath $phase3Root
 $env:BARRIER_ATLAS_CDP_PORT = "$phase3Port"
 $env:BARRIER_ATLAS_SMOKE_OUTPUT = Join-Path $phase3Root '.runtime/phase3-walkthrough'
 $env:BARRIER_ATLAS_RESULT_PATH = Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'phase2-regression.json'
 $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$phase3Port"
 $env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $phase3Root '.runtime/phase3-webview'
 $phase3Process = Start-Process -FilePath $phase3Exe -WindowStyle Hidden -PassThru
 node scripts/check_phase2_native.mjs
 $phase3Exit = $LASTEXITCODE
 if ($phase3Exit -eq 0) { node scripts/check_phase3_native.mjs; $phase3Exit = $LASTEXITCODE }
} finally {
 if ($phase3Process -and -not $phase3Process.HasExited) { Stop-Process -Id $phase3Process.Id }
 foreach ($phase3Name in $phase3Variables) { [Environment]::SetEnvironmentVariable($phase3Name,$phase3Previous[$phase3Name],'Process') }
 Set-Location -LiteralPath $phase3Directory.Path
}
exit $phase3Exit
