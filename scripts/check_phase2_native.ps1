# Run from repository root, after npm run tauri build -- --no-bundle in app/.
$ErrorActionPreference = 'Stop'
$phase2Root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$phase2Port = 9333
if (Get-NetTCPConnection -LocalPort $phase2Port -State Listen -ErrorAction SilentlyContinue) {
    throw "Diagnostic port $phase2Port is occupied; stop that session before testing."
}
$phase2Exe = Join-Path $phase2Root 'app/src-tauri/target/release/barrier-atlas.exe'
if (-not (Test-Path -LiteralPath $phase2Exe)) { throw 'Build the original application first.' }
$phase2PreviousDirectory = Get-Location
$phase2OldPort = $env:BARRIER_ATLAS_CDP_PORT
$phase2OldArguments = $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS
$phase2OldFolder = $env:WEBVIEW2_USER_DATA_FOLDER
try {
    Set-Location -LiteralPath $phase2Root
    $env:BARRIER_ATLAS_CDP_PORT = "$phase2Port"
    $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$phase2Port"
    $env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $phase2Root '.runtime/phase2-webview'
    $phase2Process = Start-Process -FilePath $phase2Exe -WindowStyle Hidden -PassThru
    node scripts/check_phase2_native.mjs
    $phase2ExitCode = $LASTEXITCODE
} finally {
    if ($phase2Process -and -not $phase2Process.HasExited) { Stop-Process -Id $phase2Process.Id }
    $env:BARRIER_ATLAS_CDP_PORT = $phase2OldPort
    $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = $phase2OldArguments
    $env:WEBVIEW2_USER_DATA_FOLDER = $phase2OldFolder
    Set-Location -LiteralPath $phase2PreviousDirectory.Path
}
exit $phase2ExitCode
