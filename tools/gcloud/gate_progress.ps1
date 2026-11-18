# Live progress bars for every gate run on both VMs. Read-only (ssh + ps), safe while agents gate.
#   powershell -ExecutionPolicy Bypass -File C:\Users\karti\sumo-rs\tools\gcloud\gate_progress.ps1
param([int]$Every = 15, [switch]$Once)
# user's order: only the personal account, never the work account
$env:CLOUDSDK_CORE_ACCOUNT = "kartikeytilavat6819@gmail.com"
$env:CLOUDSDK_CORE_PROJECT = "project-b17b0335-3081-4c61-89b"
$vms = @(@{ n = "sumo-gate"; z = "us-central1-a" }, @{ n = "sumo-gate2"; z = "us-central1-b" })
$steps = @("build", "run_all", "depart", "duarouter", "unit tests", "agent_run")
$b64 = [Convert]::ToBase64String([IO.File]::ReadAllBytes((Join-Path $PSScriptRoot "gate_progress_remote.sh")))
function Bar([int]$pct) { $n = [int]($pct / 4); "[" + ("#" * $n) + ("." * (25 - $n)) + "] {0,3}%" -f $pct }
# Kaggle (3 accounts): dispatcher, queue, and the 3 most recently run kernels per account. Slow CLI, so every ~5 min.
function KaggleStatus {
    $o = @("kaggle (as of $(Get-Date -Format HH:mm), refreshed every ~5 min)")
    $disp = @(Get-CimInstance Win32_Process -Filter "Name='powershell.exe'" | Where-Object { $_.CommandLine -match 'gate_dispatcher' }).Count
    $reqs = @(Get-ChildItem "$HOME\.kaggle\gateq\*.req" -ErrorAction SilentlyContinue).Count
    $o += "  dispatcher: $(if ($disp) { 'RUNNING' } else { 'stopped' })   queued requests: $reqs"
    foreach ($tf in "gate_token", "gate_token_acct3", "gate_token_acct1") {
        $env:KAGGLE_API_TOKEN = (Get-Content "$HOME\.kaggle\$tf" -Raw).Trim()
        foreach ($k in @(python -m kaggle kernels list --mine --sort-by dateRun --page-size 3 --csv 2>$null | ConvertFrom-Csv)) {
            $raw = python -m kaggle kernels status $k.ref 2>&1 | Out-String
            $st = if ($raw -match 'has status\s+"?(KernelWorkerStatus\.)?([A-Za-z_]+)') { $Matches[2] } else { "?" }
            $o += "  {0,-34} {1,-9} last run {2}" -f $k.ref, $st.Trim(), $k.lastRunTime.Substring(0, 16)
        }
    }
    Remove-Item Env:KAGGLE_API_TOKEN -ErrorAction SilentlyContinue
    $o
}
$tick = 0
while ($true) {
    if ($tick++ % [math]::Max([int](300 / $Every), 1) -eq 0) { $kaggle = KaggleStatus }
    $screen = @("gate progress  $(Get-Date -Format HH:mm:ss)  (refresh ${Every}s, Ctrl+C to quit)", "")
    foreach ($v in $vms) {
        $st = gcloud compute instances describe $v.n --zone $v.z --format "value(status)" 2>$null
        if ($st -ne "RUNNING") { $screen += "$($v.n): $st", ""; continue }
        $out = @(gcloud compute ssh $v.n --zone $v.z --quiet --command "echo $b64 | base64 -d | tr -d '\r' | sudo bash" 2>$null)
        $disk = ($out | Where-Object { $_ -like "DISK|*" }) -replace "^DISK\|", ""
        $runs = @($out | Where-Object { $_ -match "^\d*\|" })
        $screen += "$($v.n)  disk $disk  runs $($runs.Count)/8"
        foreach ($r in $runs) {
            $slot, $label, $et, $step, $done, $total, $detail = $r -split "\|"
            $k = [array]::IndexOf($steps, $step) + 1
            $time = "{0:mm\:ss}" -f [timespan]::FromSeconds([int]$et)
            $screen += "  slot $slot  $label  $time  step ${k}: $step"
            if ($step -eq "run_all" -and [int]$total -gt 0) {
                $screen += "    " + (Bar ([int](100 * [int]$done / [int]$total))) + "  case $done/$total  $detail"
            } elseif ($detail) { $screen += "    $detail" }
        }
        $screen += ""
    }
    $screen += @(python (Join-Path $PSScriptRoot "gate_cost.py") 2>&1 | ForEach-Object { "$_" })
    $screen += "", $kaggle
    # OpenCode free-model agents (tools/lead/oc_agent.sh): last line of each log
    $oc = @(Get-ChildItem "$PSScriptRoot\..\..\.claude\oc-*.log" -ErrorAction SilentlyContinue)
    if ($oc) {
        $free = [int]((Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory / 1KB)
        $screen += "", "opencode agents (laptop free RAM $free MB)"
        foreach ($f in $oc) { $screen += "  $($f.BaseName): " + ((Get-Content $f.FullName -Tail 1) -replace '^(.{0,150}).*', '$1') }
    }
    Clear-Host; $screen | ForEach-Object { $_ }
    if ($Once) { break }
    Start-Sleep $Every
}
