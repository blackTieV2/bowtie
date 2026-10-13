param([string]$EvidenceRoot='runs/008-build-phase7',[string]$RuntimeRoot='.runtime/phase7-walkthrough',[switch]$IncidentOnly)
$ErrorActionPreference='Stop'
$auditRoot=(Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$auditPort=9338
if(Get-NetTCPConnection -LocalPort $auditPort -State Listen -ErrorAction SilentlyContinue){throw "Diagnostic port $auditPort is occupied."}
$auditDirectory=Get-Location
$auditVariables=@('BARRIER_ATLAS_CDP_PORT','BARRIER_ATLAS_SMOKE_OUTPUT','BARRIER_ATLAS_RESULT_PATH','BARRIER_ATLAS_PHASE3_RUN','BARRIER_ATLAS_PHASE6_RUN','BARRIER_ATLAS_PHASE7_RUN','WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS','WEBVIEW2_USER_DATA_FOLDER')
$auditPrevious=@{}
foreach($auditName in $auditVariables){$auditPrevious[$auditName]=[Environment]::GetEnvironmentVariable($auditName,'Process')}
try {
 Set-Location -LiteralPath $auditRoot
 $env:BARRIER_ATLAS_CDP_PORT="$auditPort"
 $env:BARRIER_ATLAS_SMOKE_OUTPUT=Join-Path $auditRoot $RuntimeRoot
 $env:BARRIER_ATLAS_RESULT_PATH=Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'phase2-regression.json'
 $env:BARRIER_ATLAS_PHASE3_RUN=Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'regression'
 $env:BARRIER_ATLAS_PHASE6_RUN=Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'phase6-regression'
 $env:BARRIER_ATLAS_PHASE7_RUN=Join-Path $auditRoot $EvidenceRoot
 $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=$auditPort"
 $env:WEBVIEW2_USER_DATA_FOLDER=Join-Path $env:BARRIER_ATLAS_SMOKE_OUTPUT 'webview'
 $auditProcess=Start-Process -FilePath (Join-Path $auditRoot 'app/src-tauri/target/release/barrier-atlas.exe') -WindowStyle Hidden -PassThru
 $auditExit=0
 if(-not $IncidentOnly){node scripts/check_phase2_native.mjs;$auditExit=$LASTEXITCODE;if($auditExit -eq 0){node scripts/check_phase3_native.mjs;$auditExit=$LASTEXITCODE}}
 if(-not $IncidentOnly -and $auditExit -eq 0){node scripts/check_phase6_native.mjs;$auditExit=$LASTEXITCODE}
 if($auditExit -eq 0){node scripts/check_phase7_native.mjs;$auditExit=$LASTEXITCODE}
}finally{
 if($auditProcess -and -not $auditProcess.HasExited){Stop-Process -Id $auditProcess.Id}
 foreach($auditName in $auditVariables){[Environment]::SetEnvironmentVariable($auditName,$auditPrevious[$auditName],'Process')}
 Set-Location -LiteralPath $auditDirectory.Path
}
exit $auditExit
