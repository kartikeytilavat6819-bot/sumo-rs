# Gate one branch on the Google Cloud VM (16 cores, warm target dir): no queue, ~2-5 min incremental.
#   powershell -ExecutionPolicy Bypass -File C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1 -Branch wave2
# Paid from the project's welcome credit; the VM stops itself after 30 idle minutes.
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [string]$Vm = ""
)
$ErrorActionPreference = "Continue"
# user's order: only the personal account, never the work account
$env:CLOUDSDK_CORE_ACCOUNT = "kartikeytilavat6819@gmail.com"
$env:CLOUDSDK_CORE_PROJECT = "project-b17b0335-3081-4c61-89b"
# two VMs; slots are first-free (no per-branch warmth), so go to the VM with fewer runs from this machine
$vms = @(@{ n = "sumo-gate"; z = "us-central1-a" }, @{ n = "sumo-gate2"; z = "us-central1-b" })
$mark = Join-Path $env:TEMP "vmgate_runs"
New-Item -ItemType Directory -Force $mark | Out-Null
$sha = (git -C $Repo rev-parse --short $Branch).Trim()
if (-not $sha) { "unknown branch $Branch"; exit 1 }
$id = (("$Branch-$sha".ToLower()) -replace '[^a-z0-9]+', '-').Trim('-')
# user's order: no idle capacity. A free VM slot (8 per VM) wins; with all 16 busy the gate is queued for the Kaggle
# dispatcher too, and whichever starts first runs it (a still-untaken Kaggle request is withdrawn when a VM slot frees).
# Branches whose agent_run.sh needs instrumented C++ (vm_cpp_trace.sh, VM only) never go to Kaggle.
$slots = 8
$q = "$HOME\.kaggle\gateq"; $kreq = "$q\$id.req"; $kres = "$q\$id.result"
$vmOnly = [bool]((git -C $Repo show "${sha}:tools/kaggle/agent_run.sh" 2>$null) -match 'vm_cpp_trace') -or ($Branch -eq 'wave2')  # wave2 decides main: the Kaggle runner skips the duarouter/netopts/tool oracles
$queued = $false
while (-not $Vm) {
    Get-ChildItem $mark -File | Where-Object { -not (Get-Process -Id ([int]($_.Name -split '_')[-1]) -ErrorAction SilentlyContinue) } | Remove-Item -ErrorAction SilentlyContinue
    $free = $vms | Where-Object { @(Get-ChildItem $mark -Filter "$($_.n)_*").Count -lt $slots } |
        Sort-Object { @(Get-ChildItem $mark -Filter "$($_.n)_*").Count }, { $_.n } | Select-Object -First 1
    if ($free) {
        # withdraw the Kaggle request unless the dispatcher already took it (rename is atomic: one side wins)
        if ($queued -and -not (Rename-Item $kreq "$id.req.withdrawn" -PassThru -ErrorAction SilentlyContinue)) { break }
        Remove-Item "$q\$id.req.withdrawn" -ErrorAction SilentlyContinue
        $Vm = $free.n; break
    }
    if (-not $vmOnly -and -not $queued) {
        New-Item -ItemType Directory -Force $q | Out-Null
        if (-not (Test-Path $kres)) { Set-Content $kreq "$Branch $sha $Repo" }
        $queued = $true; "vm_gate: all $($slots * $vms.Count) VM slots busy; queued on Kaggle too (first free side runs it)"
    }
    if (Test-Path $kres) { break }
    Start-Sleep 20
}
if (-not $Vm) {
    # Kaggle took it: wait for its result (same format as the VM gate: "<branch> <sha>" first)
    "vm_gate: running on Kaggle"
    $deadline = (Get-Date).AddMinutes(180)
    while (-not (Test-Path $kres) -and (Get-Date) -lt $deadline) { Start-Sleep 20 }
    if (-not (Test-Path $kres)) { "NO RESULT from Kaggle after 180 min"; exit 1 }
    Get-Content $kres; exit 0
}
$markFile = Join-Path $mark "${Vm}_$PID"
New-Item -ItemType File -Force $markFile | Out-Null
$Zone = ($vms | Where-Object { $_.n -eq $Vm }).z
$tgz = Join-Path $env:TEMP "$id.tgz"
git -C $Repo archive --format=tar.gz -o $tgz $sha
$st = gcloud compute instances describe $Vm --zone $Zone --format "value(status)" 2>$null
if ($st -ne "RUNNING") { gcloud compute instances start $Vm --zone $Zone --quiet 2>&1 | Out-Null; Start-Sleep 30 }
$run = Join-Path $PSScriptRoot "vm_gate_run.sh"
# an ssh/scp drop (many agents connect at once) loses the whole run: retry until the run script reports an end
for ($try = 1; $try -le 3; $try++) {
    gcloud compute scp $tgz $run "${Vm}:/tmp/" --zone $Zone --quiet 2>&1 | Out-Null
    $out = @(gcloud compute ssh $Vm --zone $Zone --quiet --command "while [ ! -f /opt/gate_ready ]; do sleep 10; done; sudo cp /tmp/$id.tgz /home/gate/ && sudo cp /tmp/vm_gate_run.sh /home/gate/run_$id.sh && sudo sed -i 's/\r$//' /home/gate/run_$id.sh && sudo chown gate /home/gate/$id.tgz /home/gate/run_$id.sh && sudo -iu gate bash /home/gate/run_$id.sh /home/gate/$id.tgz '$Branch $sha'; sudo rm -f /home/gate/$id.tgz /home/gate/run_$id.sh" 2>&1 |
        Where-Object { $_ -notmatch '^(Updating|Warning: Permanently|External IP|Writing|.*NativeCommandError|.*CategoryInfo|.*At .*gcloud\.ps1|\s*\+|\s*~)' })
    if (($out -join "`n") -match 'GATE DONE|BUILD FAILED') { $out; break }
    "vm_gate: attempt $try ended without a result (ssh/scp drop); retrying"
    if ($try -eq 3) { $out }
    Start-Sleep (15 * $try)
}
Remove-Item $tgz, $markFile -ErrorAction SilentlyContinue
