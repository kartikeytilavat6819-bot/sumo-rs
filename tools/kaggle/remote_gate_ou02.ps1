# Run the full oracle gate for one git branch on a Kaggle CPU notebook, off this machine's RAM.
#   powershell -File C:\Users\karti\sumo-rs\tools\kaggle\remote_gate.ps1 -Branch worktree-agent-x
# Token: a Kaggle API token (KGAT_...) for an internet-enabled account, in a file outside the repo.
# Each branch gets its own dataset+kernel pair (named after the branch), so concurrent gates of different
# branches can never overwrite each other's snapshot. The result is rejected unless the kernel reports the
# exact commit that was uploaded.
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    # optional override; default is derived from the branch name (one slot per branch)
    [string]$Slot = "",
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [int]$TimeoutMin = 90,
    # gate_token = account kartikey6819, gate_token_acct3 = kartikey5656 (both have notebook internet)
    [string]$TokenFile = "$HOME\.kaggle\gate_token"
)
# Continue: the kaggle CLI writes expected errors (e.g. 403 for a dataset that does not exist yet) to stderr
$ErrorActionPreference = "Continue"
if ($Slot -eq "") { $Slot = $Branch }
$Slot = (($Slot.ToLower() -replace '[^a-z0-9]+', '-') -replace '^worktree-agent-', '').Trim('-')
if ($Slot.Length -gt 30) { $Slot = $Slot.Substring(0, 30).Trim('-') }
$env:KAGGLE_API_TOKEN = (Get-Content $TokenFile -Raw).Trim()
$user = ((python -m kaggle config view 2>&1 | Select-String 'username') -split ':')[-1].Trim()
$work = Join-Path $env:TEMP "kaggle_gate_$Slot"
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
$ds = "$user/srg-src-$Slot"
@{ title = "srg-src-$Slot"; id = $ds; licenses = @(@{ name = "CC0-1.0" }) } | ConvertTo-Json | Set-Content "$work\data\dataset-metadata.json"
$exists = python -m kaggle datasets status $ds 2>&1 | Out-String
if ($exists -match 'ready|pending') { python -m kaggle datasets version -p "$work\data" -m "$Branch $sha" -q | Out-Null }
else { python -m kaggle datasets create -p "$work\data" -q | Out-Null }
for ($i = 0; $i -lt 40; $i++) { if ((python -m kaggle datasets status $ds 2>&1 | Out-String) -match 'ready') { break }; Start-Sleep 10 }

# 3. run the gate kernel
$kernel = "$user/srg-$Slot"
Copy-Item (Join-Path $PSScriptRoot "gate_kernel_ou02.py") "$work\kernel\gate_kernel_ou02.py"
@{ id = $kernel; title = "srg-$Slot"; code_file = "gate_kernel_ou02.py"; language = "python";
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

# 4. fetch, verify it tested the uploaded commit, print the summary
python -m kaggle kernels output $kernel -p "$work\out" | Out-Null
"branch: $Branch $sha  kernel: $kernel $($st.Trim())"
if (-not (Test-Path "$work\out\gate.txt")) { "NO RESULT - no gate.txt in $work\out"; exit 1 }
$gate = Get-Content "$work\out\gate.txt"
if (-not ($gate -match [regex]::Escape("$Branch $sha"))) {
    "WRONG SNAPSHOT - the kernel did not test $Branch $sha (stale dataset or another run). Do not use this result; rerun."
    exit 1
}
$gate
