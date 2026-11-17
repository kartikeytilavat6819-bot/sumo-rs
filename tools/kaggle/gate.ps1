# Batched remote gate (client). Queues the branch for the dispatcher and waits for its result.
#   powershell -ExecutionPolicy Bypass -File C:\Users\karti\sumo-rs\tools\kaggle\gate.ps1 -Branch worktree-agent-x
# The dispatcher (gate_dispatcher.ps1, run once by the lead) packs up to 5 queued branches into one Kaggle
# notebook that builds them one after another with a shared target dir, so each extra branch costs ~1-2 min
# instead of a whole session. Results are tied to the exact commit: "<branch> <sha>" is the first line.
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [int]$TimeoutMin = 180
)
$ErrorActionPreference = "Continue"
$q = "$HOME\.kaggle\gateq"
New-Item -ItemType Directory -Force $q | Out-Null
$sha = (git -C $Repo rev-parse --short $Branch).Trim()
if (-not $sha) { "unknown branch $Branch"; exit 1 }
$id = (("$Branch-$sha".ToLower()) -replace '[^a-z0-9]+', '-').Trim('-')
$res = "$q\$id.result"
if (-not (Test-Path $res)) { Set-Content "$q\$id.req" "$Branch $sha $Repo" }
"queued $Branch $sha as $id; waiting for the dispatcher..."
$deadline = (Get-Date).AddMinutes($TimeoutMin)
while (-not (Test-Path $res) -and (Get-Date) -lt $deadline) { Start-Sleep 20 }
if (-not (Test-Path $res)) { "NO RESULT after $TimeoutMin min (is gate_dispatcher.ps1 running?)"; exit 1 }
$gate = Get-Content $res
if ($gate[0] -ne "$Branch $sha") { "WRONG SNAPSHOT - result is for '$($gate[0])', expected '$Branch $sha'"; exit 1 }
$gate
