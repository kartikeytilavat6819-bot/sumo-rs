# Copy of remote_gate.ps1 for the emissions branch: uploads the PHEMlight data (sumo_data/) next to the
# source snapshot and runs gate_kernel_em02.py, which sets SUMO_HOME to it. The shared gate is unchanged.
#   powershell -File C:\Users\karti\sumo-rs\tools\kaggle\remote_gate_em02.ps1 -Branch worktree-agent-em02
# PHEMlight data source: the pristine SUMO install (WSL), copied to the temp dir below. Re-copy it there if missing:
#   wsl -e bash -lc 'D=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo/data/emissions; cp -r $D/PHEMlight $D/PHEMlight5 /mnt/c/Users/karti/AppData/Local/Temp/opencode/em02_phem/'
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    [string]$Slot = "",
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [int]$TimeoutMin = 90,
    [string]$TokenFile = "$HOME\.kaggle\gate_token",
    [string]$PhemData = "C:\Users\karti\AppData\Local\Temp\opencode\em02_phem"
)
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

# 1b. PHEMlight data for SUMO_HOME (sumo_data/data/emissions/PHEMlight{,5})
$emis = "$work\data\sumo_data\data\emissions"
New-Item -ItemType Directory -Force $emis | Out-Null
Copy-Item -Recurse "$PhemData\PHEMlight", "$PhemData\PHEMlight5" $emis
if (-not (Test-Path "$emis\PHEMlight\PC_G_EU4.csv")) { "NO PHEMlight data in $PhemData"; exit 1 }

# 2. upload as a private dataset (create on first use, new version afterwards)
$ds = "$user/srg-src-$Slot"
@{ title = "srg-src-$Slot"; id = $ds; licenses = @(@{ name = "CC0-1.0" }) } | ConvertTo-Json | Set-Content "$work\data\dataset-metadata.json"
$exists = python -m kaggle datasets status $ds 2>&1 | Out-String
if ($exists -match 'ready|pending') { python -m kaggle datasets version -p "$work\data" -m "$Branch $sha" -q | Out-Null }
else { python -m kaggle datasets create -p "$work\data" -q | Out-Null }
for ($i = 0; $i -lt 40; $i++) { if ((python -m kaggle datasets status $ds 2>&1 | Out-String) -match 'ready') { break }; Start-Sleep 10 }

# 3. run the gate kernel (em02 copy: sets SUMO_HOME)
$kernel = "$user/srg-$Slot"
Copy-Item (Join-Path $PSScriptRoot "gate_kernel_em02.py") "$work\kernel\gate_kernel.py"
@{ id = $kernel; title = "srg-$Slot"; code_file = "gate_kernel.py"; language = "python";
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
