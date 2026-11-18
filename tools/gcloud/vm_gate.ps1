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
$full = (git -C $Repo rev-parse $sha).Trim()
# official-suite tt_ cases this branch adds or changes: an agent gate runs only these, wave2 runs all of them
$ttf = Join-Path $env:TEMP "$id.ttf"
$ttKey = "all"
if ($Branch -ne 'wave2') {
    $mb = (git -C $Repo merge-base main $full).Trim()
    $tt = @(git -C $Repo diff --name-only $mb $full -- tools/oracle/cases/ | ForEach-Object { if ($_ -match '^tools/oracle/cases/(tt_[^/]+)/') { $Matches[1] } } | Sort-Object -Unique)
    [IO.File]::WriteAllText($ttf, ($tt -join "`n"))
    $ttKey = [BitConverter]::ToString([Security.Cryptography.SHA1]::Create().ComputeHash([Text.Encoding]::UTF8.GetBytes($tt -join "`n"))).Replace('-', '').Substring(0, 12)
}
# Identical gates (same commit, same tt_ case set; e.g. several agents' baseline gates of one base) run once:
# the first takes a lock, the others wait for its result and reuse it.
$cdir = Join-Path $env:TEMP "vmgate_cache"
New-Item -ItemType Directory -Force $cdir | Out-Null
Get-ChildItem $cdir -File | Where-Object { $_.LastWriteTime -lt (Get-Date).AddDays(-2) } | Remove-Item -ErrorAction SilentlyContinue
# the gate scripts are part of the key: fixing them must not hand back a result they produced
$infra = [BitConverter]::ToString([Security.Cryptography.SHA1]::Create().ComputeHash([IO.File]::ReadAllBytes((Join-Path $PSScriptRoot "vm_gate_run.sh")) + [IO.File]::ReadAllBytes((Join-Path $PSScriptRoot "..\oracle\run_all.sh")))).Replace('-', '').Substring(0, 8)
$cout = Join-Path $cdir "$full-$ttKey-$infra.out"; $clock = Join-Path $cdir "$full-$ttKey-$infra.lock"
while ($true) {
    if (Test-Path $cout) { "vm_gate: reusing the result of an identical gate (same commit and case set)"; Get-Content $cout; exit 0 }
    try {
        $fs = [IO.File]::Open($clock, 'CreateNew', 'Write'); $w = New-Object IO.StreamWriter($fs); $w.Write("$PID"); $w.Close(); break
    } catch {
        $owner = 0; [int]::TryParse(((Get-Content $clock -ErrorAction SilentlyContinue) -join ''), [ref]$owner) | Out-Null
        $age = ((Get-Date) - (Get-Item $clock -ErrorAction SilentlyContinue).LastWriteTime).TotalSeconds
        if (($owner -and -not (Get-Process -Id $owner -ErrorAction SilentlyContinue)) -or (-not $owner -and $age -gt 30)) { Remove-Item $clock -ErrorAction SilentlyContinue; continue }
        Start-Sleep 20
    }
}
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
    if (-not (Test-Path $kres)) { "NO RESULT from Kaggle after 180 min"; Remove-Item $clock -ErrorAction SilentlyContinue; exit 1 }
    Remove-Item $clock -ErrorAction SilentlyContinue
    Get-Content $kres; exit 0
}
$markFile = Join-Path $mark "${Vm}_$PID"
New-Item -ItemType File -Force $markFile | Out-Null
$Zone = ($vms | Where-Object { $_.n -eq $Vm }).z
# Upload only the commits the VM's git cache (~/gaterepo.git) lacks: a full snapshot is ~100 MB and 16 parallel
# uploads saturated the line (truncated/hung uploads). The VM then builds the snapshot tgz from its cache.
$bdl = Join-Path $env:TEMP "$id.bundle"
git -C $Repo update-ref "refs/gate/$id" $full
$st = gcloud compute instances describe $Vm --zone $Zone --format "value(status)" 2>$null
if ($st -ne "RUNNING") { gcloud compute instances start $Vm --zone $Zone --quiet 2>&1 | Out-Null; Start-Sleep 30 }
$run = Join-Path $PSScriptRoot "vm_gate_run.sh"
$ra = Join-Path $PSScriptRoot "..\oracle\run_all.sh"   # lead's sharded/filtered run_all, used when the branch's copy predates it
# an ssh/scp drop (many agents connect at once) loses the whole run: retry until the run script reports an end
for ($try = 1; $try -le 3; $try++) {
    $known = @(gcloud compute ssh $Vm --zone $Zone --quiet --command "git -C ~/gaterepo.git for-each-ref --sort=-creatordate --count=80 --format='%(objectname)' 2>/dev/null" 2>$null) |
        Where-Object { $_ -match '^[0-9a-f]{40}$' }
    $not = @($known | Where-Object { git -C $Repo cat-file -e "$_^{commit}" 2>$null; $LASTEXITCODE -eq 0 } | ForEach-Object { "^$_" })
    Remove-Item $bdl -ErrorAction SilentlyContinue
    git -C $Repo bundle create $bdl "refs/gate/$id" @not 2>$null | Out-Null   # no file = the VM already has the commit
    $up = @($run, $ra) + @(Get-Item $bdl, $ttf -ErrorAction SilentlyContinue | ForEach-Object { $_.FullName })
    gcloud compute scp @up "${Vm}:/tmp/" --zone $Zone --quiet 2>&1 | Out-Null
    $prep = "R=~/gaterepo.git; [ -d `$R ] || git init -q --bare `$R; if [ -f /tmp/$id.bundle ]; then git -C `$R fetch -q /tmp/$id.bundle '+refs/gate/*:refs/gate/*' && rm -f /tmp/$id.bundle; fi; git -C `$R archive --format=tar.gz -o /tmp/$id.tgz $full || echo 'SNAPSHOT CORRUPT (missing commit)';"
    $out = @(gcloud compute ssh $Vm --zone $Zone --quiet --command "$prep while [ ! -f /opt/gate_ready ]; do sleep 10; done; sudo cp /tmp/run_all.sh /home/gate/lead_run_all.sh; [ -f /tmp/$id.ttf ] && sudo cp /tmp/$id.ttf /home/gate/ttf_$id; sudo cp /tmp/$id.tgz /home/gate/ && sudo cp /tmp/vm_gate_run.sh /home/gate/run_$id.sh && sudo sed -i 's/\r$//' /home/gate/run_$id.sh && sudo chown gate /home/gate/$id.tgz /home/gate/run_$id.sh && sudo -iu gate bash /home/gate/run_$id.sh /home/gate/$id.tgz '$Branch $sha' /home/gate/ttf_$id; sudo rm -f /home/gate/$id.tgz /home/gate/run_$id.sh /home/gate/ttf_$id /tmp/$id.tgz /tmp/$id.ttf" 2>&1 |
        Where-Object { $_ -notmatch '^(Updating|Warning: Permanently|External IP|Writing|.*NativeCommandError|.*CategoryInfo|.*At .*gcloud\.ps1|\s*\+|\s*~)' })
    if (($out -join "`n") -match 'GATE DONE|BUILD FAILED') { $out; $out | Set-Content $cout; break }
    "vm_gate: attempt $try ended without a result (ssh/scp drop); retrying"
    if ($try -eq 3) { $out }
    Start-Sleep (15 * $try)
}
git -C $Repo update-ref -d "refs/gate/$id"
Remove-Item $bdl, $ttf, $markFile, $clock -ErrorAction SilentlyContinue
