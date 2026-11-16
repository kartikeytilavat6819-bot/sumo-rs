# st01 round-trip run on Kaggle: snapshot the branch, push roundtrip_kernel_st01.py, fetch rt/ outputs.
#   powershell -File tools/kaggle/roundtrip_st01.ps1 -Branch worktree-agent-st01 -TokenFile $HOME\.kaggle\gate_token
# Names carry the agent id (st01), not a slot number, so they cannot collide with other agents' runs.
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [int]$TimeoutMin = 90,
    [string]$TokenFile = "$HOME\.kaggle\gate_token"
)
$ErrorActionPreference = "Continue"
$env:KAGGLE_API_TOKEN = (Get-Content $TokenFile -Raw).Trim()
$user = ((python -m kaggle config view 2>&1 | Select-String 'username') -split ':')[-1].Trim()
$work = Join-Path $env:TEMP "kaggle_rt_st01"
Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force "$work\data", "$work\kernel", "$work\out" | Out-Null

$sha = (git -C $Repo rev-parse --short $Branch).Trim()
Set-Content "$work\GATE_BRANCH.txt" "$Branch $sha"
git -C $Repo archive --format=tar --prefix= -o "$work\src.tar" $Branch
$p = "$(($work -replace '\\','/') -replace '^C:','/c')"
& 'C:\Program Files\Git\usr\bin\tar.exe' -rf "$p/src.tar" -C $p GATE_BRANCH.txt
& 'C:\Program Files\Git\usr\bin\gzip.exe' -f "$p/src.tar"
Move-Item "$work\src.tar.gz" "$work\data\src.tgz"

$ds = "$user/sumo-rs-rt-st01-src"
@{ title = "sumo-rs-rt-st01-src"; id = $ds; licenses = @(@{ name = "CC0-1.0" }) } | ConvertTo-Json | Set-Content "$work\data\dataset-metadata.json"
$exists = python -m kaggle datasets status $ds 2>&1 | Out-String
if ($exists -match 'ready|pending') { python -m kaggle datasets version -p "$work\data" -m "$Branch $sha" -q | Out-Null }
else { python -m kaggle datasets create -p "$work\data" -q | Out-Null }
for ($i = 0; $i -lt 40; $i++) { if ((python -m kaggle datasets status $ds 2>&1 | Out-String) -match 'ready') { break }; Start-Sleep 10 }

$kernel = "$user/sumo-rs-rt-st01"
Copy-Item (Join-Path $PSScriptRoot "roundtrip_kernel_st01.py") "$work\kernel\roundtrip_kernel_st01.py"
@{ id = $kernel; title = "sumo-rs-rt-st01"; code_file = "roundtrip_kernel_st01.py"; language = "python";
   kernel_type = "script"; is_private = $true; enable_gpu = $false; enable_internet = $true;
   dataset_sources = @($ds); competition_sources = @(); kernel_sources = @() } | ConvertTo-Json | Set-Content "$work\kernel\kernel-metadata.json"
for ($t = 0; $t -lt 10; $t++) {
    $push = python -m kaggle kernels push -p "$work\kernel" 2>&1 | Out-String
    if ($push -notmatch 'refused|Maximum') { break }
    Start-Sleep 60
}
$deadline = (Get-Date).AddMinutes($TimeoutMin)
do { Start-Sleep 30; $st = python -m kaggle kernels status $kernel 2>&1 | Out-String }
while ($st -notmatch 'COMPLETE|ERROR|CANCEL|denied|404' -and (Get-Date) -lt $deadline)

python -m kaggle kernels output $kernel -p "$work\out" | Out-Null
"branch: $Branch $sha  kernel: $($st.Trim())"
if (Test-Path "$work\out\GATE_BRANCH.txt") { Get-Content "$work\out\GATE_BRANCH.txt" }
if (Test-Path "$work\out\roundtrip.txt") { Get-Content "$work\out\roundtrip.txt" -Tail 40 } else { "no roundtrip.txt - see $work\out" }
