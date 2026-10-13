$ErrorActionPreference = 'Stop'
$compareRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$comparePort = 9336
if (Get-NetTCPConnection -LocalPort $comparePort -State Listen -ErrorAction SilentlyContinue) { throw "Diagnostic port $comparePort is occupied." }
$compareDirectory = Get-Location
$compareVariables = @('BARRIER_ATLAS_CDP_PORT','WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS','WEBVIEW2_USER_DATA_FOLDER')
$comparePrevious = @{}
foreach ($compareName in $compareVariables) { $comparePrevious[$compareName] = [Environment]::GetEnvironmentVariable($compareName, 'Process') }
try {
 Set-Location -LiteralPath $compareRoot
 $env:BARRIER_ATLAS_CDP_PORT = "$comparePort"
 $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$comparePort"
 $env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $compareRoot '.runtime/phase5-comparison-webview'
 $compareProcess = Start-Process -FilePath (Join-Path $compareRoot 'app/src-tauri/target/release/barrier-atlas.exe') -WindowStyle Hidden -PassThru
 node scripts/check_phase5_compare_native.mjs
 $compareExit = $LASTEXITCODE
} finally {
 if ($compareProcess -and -not $compareProcess.HasExited) { Stop-Process -Id $compareProcess.Id }
 foreach ($compareName in $compareVariables) { [Environment]::SetEnvironmentVariable($compareName,$comparePrevious[$compareName],'Process') }
 Set-Location -LiteralPath $compareDirectory.Path
}
exit $compareExit
