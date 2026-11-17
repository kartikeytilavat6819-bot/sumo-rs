# Benchmark our simulator against C++ SUMO on one Kaggle CPU notebook (same machine, same inputs).
#   powershell -File C:\Users\karti\sumo-rs\tools\kaggle\remote_bench.ps1 -Branch worktree-agent-pf01
# Copy of remote_gate.ps1 that runs bench_kernel.py instead of gate_kernel.py. Kernel and dataset are named
# srb-*/srb-src-* so a bench run never collides with a gate run of the same branch.
# Token: a Kaggle API token (KGAT_...) for an internet-enabled account, in a file outside the repo.
# The result is rejected unless the kernel reports the exact commit that was uploaded.
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    # optional override; default is derived from the branch name (one slot per branch)
    [string]$Slot = "",
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [int]$TimeoutMin = 300,
    # gate_token_acct3 = kartikey5656 (internet-enabled account)
    [string]$TokenFile = "$HOME\.kaggle\gate_token_acct3"
)
# Continue: the kaggle CLI writes expected errors (e.g. 403 for a dataset that does not exist yet) to stderr
$ErrorActionPreference = "Continue"
if ($Slot -eq "") { $Slot = $Branch }
$Slot = (($Slot.ToLower() -replace '[^a-z0-9]+', '-') -replace '^worktree-agent-', '').Trim('-')
if ($Slot.Length -gt 30) { $Slot = $Slot.Substring(0, 30).Trim('-') }
$env:KAGGLE_API_TOKEN = (Get-Content $TokenFile -Raw).Trim()
$user = ((python -m kaggle config view 2>&1 | Select-String 'username') -split ':')[-1].Trim()
$work = Join-Path $env:TEMP "kaggle_bench_$Slot"
Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force "$work\data", "$work\kernel", "$work\out" | Out-Null
$wpos = ($work -replace '\\', '/') -replace '^C:', '/c'

# 1. source snapshot of the branch (plus a marker so the log says what was tested)
$sha = (git -C $Repo rev-parse --short $Branch).Trim()
Set-Content "$work\GATE_BRANCH.txt" "$Branch $sha"
git -C $Repo archive --format=tar --prefix= -o "$work\src.tar" $Branch
& 'C:\Program Files\Git\usr\bin\tar.exe' -rf "$wpos/src.tar" -C "$wpos" GATE_BRANCH.txt
& 'C:\Program Files\Git\usr\bin\gzip.exe' -f "$wpos/src.tar"
Move-Item "$work\src.tar.gz" "$work\data\src.tgz"

# 2. upload as a private dataset (create on first use, new version afterwards)
$ds = "$user/srb-src-$Slot"
@{ title = "srb-src-$Slot"; id = $ds; licenses = @(@{ name = "CC0-1.0" }) } | ConvertTo-Json | Set-Content "$work\data\dataset-metadata.json"
$exists = python -m kaggle datasets status $ds 2>&1 | Out-String
if ($exists -match 'ready|pending') { python -m kaggle datasets version -p "$work\data" -m "$Branch $sha" -q | Out-Null }
else { python -m kaggle datasets create -p "$work\data" -q | Out-Null }
for ($i = 0; $i -lt 40; $i++) { if ((python -m kaggle datasets status $ds 2>&1 | Out-String) -match 'ready') { break }; Start-Sleep 10 }

# 3. run the bench kernel
$kernel = "$user/srb-$Slot"
Copy-Item (Join-Path $PSScriptRoot "bench_kernel.py") "$work\kernel\bench_kernel.py"
@{ id = $kernel; title = "srb-$Slot"; code_file = "bench_kernel.py"; language = "python";
   kernel_type = "script"; is_private = $true; enable_gpu = $false; enable_internet = $true;
   dataset_sources = @($ds); competition_sources = @(); kernel_sources = @() } | ConvertTo-Json | Set-Content "$work\kernel\kernel-metadata.json"
$deadline = (Get-Date).AddMinutes($TimeoutMin)
# an account runs at most 5 notebooks at once; Kaggle refuses the push when all are busy, so wait and retry
do {
    $push = python -m kaggle kernels push -p "$work\kernel" 2>&1 | Out-String
    if ($push -match 'successfully pushed') { break }
    "kernel push refused, retrying in 60 s: $(($push -split "`n")[0].Trim())"
    Start-Sleep 60
} while ((Get-Date) -lt $deadline)
if ($push -notmatch 'successfully pushed') { "NO RESULT - could not start the kernel before the timeout"; exit 1 }
do { Start-Sleep 30; $st = python -m kaggle kernels status $kernel 2>&1 | Out-String }
while ($st -notmatch 'COMPLETE|ERROR|CANCEL|denied' -and (Get-Date) -lt $deadline)

# 4. fetch, verify it benchmarked the uploaded commit, print the summary
python -m kaggle kernels output $kernel -p "$work\out" | Out-Null
"branch: $Branch $sha  kernel: $kernel $($st.Trim())"
if (-not (Test-Path "$work\out\bench.txt")) { "NO RESULT - no bench.txt in $work\out"; exit 1 }
$bench = Get-Content "$work\out\bench.txt"
if (-not ($bench -match [regex]::Escape("$Branch $sha"))) {
    "WRONG SNAPSHOT - the kernel did not benchmark $Branch $sha (stale dataset or another run). Do not use this result; rerun."
    exit 1
}
if (-not (Test-Path "$work\out\results.json")) { "NO RESULT - no results.json in $work\out (see bench.txt)" }
else { Copy-Item "$work\out\results.json" (Join-Path $PSScriptRoot "bench_results_$Slot.json") -Force }
$bench
