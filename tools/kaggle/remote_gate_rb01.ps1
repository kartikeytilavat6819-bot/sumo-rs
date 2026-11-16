# Run the full oracle gate for one git branch on a Kaggle CPU notebook, off this machine's RAM.
# rb01 variant of remote_gate.ps1: pushes gate_kernel_rb01.py (also copies the rail case outputs).
#   powershell -File tools/kaggle/remote_gate_rb01.ps1 -Branch worktree-agent-x -Slot 1
# Token: put the Kaggle API token (KGAT_...) for an internet-enabled account in $HOME\.kaggle\gate_token.
# -Slot is the agent id (default rb01): names the dataset and kernel, so runs never share them.
# gate.txt must contain "<branch> <sha>" from the snapshot; the summary below is checked by hand.
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    [string]$Slot = "rb01",
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [int]$TimeoutMin = 90,
    # gate_token = account kartikey6819, gate_token_acct3 = kartikey5656 (both have notebook internet)
    [string]$TokenFile = "$HOME\.kaggle\gate_token"
)
# Continue: the kaggle CLI writes expected errors (e.g. 403 for a dataset that does not exist yet) to stderr
$ErrorActionPreference = "Continue"
$env:KAGGLE_API_TOKEN = (Get-Content $TokenFile -Raw).Trim()
$user = ((python -m kaggle config view 2>&1 | Select-String 'username') -split ':')[-1].Trim()
$work = Join-Path $env:TEMP "kaggle_gate_$Slot"
Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force "$work\data", "$work\kernel", "$work\out" | Out-Null

# 1. source snapshot of the branch (plus a marker so the log says what was tested)
$sha = (git -C $Repo rev-parse --short $Branch).Trim()
Set-Content "$work\GATE_BRANCH.txt" "$Branch $sha"
git -C $Repo archive --format=tar --prefix= -o "$work\src.tar" $Branch
& 'C:\Program Files\Git\usr\bin\tar.exe' -rf "$(($work -replace '\\','/') -replace '^C:','/c')/src.tar" -C "$(($work -replace '\\','/') -replace '^C:','/c')" GATE_BRANCH.txt
& 'C:\Program Files\Git\usr\bin\gzip.exe' -f "$(($work -replace '\\','/') -replace '^C:','/c')/src.tar"
Move-Item "$work\src.tar.gz" "$work\data\src.tgz"

# 2. upload as a private dataset (create on first use, new version afterwards)
$ds = "$user/sumo-rs-gate-src-$Slot"
@{ title = "sumo-rs-gate-src-$Slot"; id = $ds; licenses = @(@{ name = "CC0-1.0" }) } | ConvertTo-Json | Set-Content "$work\data\dataset-metadata.json"
$exists = python -m kaggle datasets status $ds 2>&1 | Out-String
if ($exists -match 'ready|pending') { python -m kaggle datasets version -p "$work\data" -m "$Branch $sha" -q | Out-Null }
else { python -m kaggle datasets create -p "$work\data" -q | Out-Null }
for ($i = 0; $i -lt 40; $i++) { if ((python -m kaggle datasets status $ds 2>&1 | Out-String) -match 'ready') { break }; Start-Sleep 10 }

# 3. run the gate kernel
Copy-Item (Join-Path $PSScriptRoot "gate_kernel_rb01.py") "$work\kernel\gate_kernel_rb01.py"
@{ id = "$user/sumo-rs-gate-$Slot"; title = "sumo-rs-gate-$Slot"; code_file = "gate_kernel_rb01.py"; language = "python";
   kernel_type = "script"; is_private = $true; enable_gpu = $false; enable_internet = $true;
   dataset_sources = @($ds); competition_sources = @(); kernel_sources = @() } | ConvertTo-Json | Set-Content "$work\kernel\kernel-metadata.json"
python -m kaggle kernels push -p "$work\kernel" | Out-Null
$deadline = (Get-Date).AddMinutes($TimeoutMin)
do { Start-Sleep 30; $st = python -m kaggle kernels status "$user/sumo-rs-gate-$Slot" 2>&1 | Out-String }
while ($st -notmatch 'COMPLETE|ERROR|CANCEL|denied' -and (Get-Date) -lt $deadline)

# 4. fetch and print the summary
python -m kaggle kernels output "$user/sumo-rs-gate-$Slot" -p "$work\out" | Out-Null
"branch: $Branch $sha  kernel: $($st.Trim())"
if (Test-Path "$work\out\gate.txt") { Get-Content "$work\out\gate.txt" } else { "no gate.txt - see $work\out" }
