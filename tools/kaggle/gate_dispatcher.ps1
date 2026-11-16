# Batched remote gate (dispatcher). Run once, in the background, by the lead:
#   powershell -ExecutionPolicy Bypass -File C:\Users\karti\sumo-rs\tools\kaggle\gate_dispatcher.ps1
# Every 30 s: collect queued requests ($HOME\.kaggle\gateq\*.req), pack up to $PerBatch snapshots into one dataset,
# run batch_kernel.py on a free worker (account x index), and turn finished outputs into <id>.result files.
param(
    [string[]]$TokenFiles = @("$HOME\.kaggle\gate_token", "$HOME\.kaggle\gate_token_acct3"),
    # leave 2 of each account's 5 sessions free for agents' own diagnostic kernels
    [int]$WorkersPerAccount = 3,
    [int]$PerBatch = 5
)
$ErrorActionPreference = "Continue"
$q = "$HOME\.kaggle\gateq"
New-Item -ItemType Directory -Force $q | Out-Null
$tar = 'C:\Program Files\Git\usr\bin\tar.exe'
function Posix($p) { ($p -replace '\\', '/') -replace '^([A-Za-z]):', { '/' + $_.Groups[1].Value.ToLower() } }
function UseToken($tf) {
    $env:KAGGLE_API_TOKEN = (Get-Content $tf -Raw).Trim()
    ((python -m kaggle config view 2>&1 | Select-String 'username') -split ':')[-1].Trim()
}
$users = @{}
foreach ($tf in $TokenFiles) { $users[$tf] = UseToken $tf }
# worker state: key "tokenfile|index" -> @{ ids = @(...); kernel = "user/name" }
$busy = @{}
"dispatcher started: accounts $($users.Values -join ', '), $WorkersPerAccount workers each, $PerBatch per batch"

while ($true) {
    # 1. finished workers -> results
    foreach ($key in @($busy.Keys)) {
        $w = $busy[$key]; $tf = $key.Split('|')[0]; UseToken $tf | Out-Null
        $st = python -m kaggle kernels status $w.kernel 2>&1 | Out-String
        if ($st -notmatch 'COMPLETE|ERROR|CANCEL') { continue }
        $out = Join-Path $env:TEMP ("gateq_out_" + ($key -replace '[^A-Za-z0-9]', '_'))
        Remove-Item -Recurse -Force $out -ErrorAction SilentlyContinue
        python -m kaggle kernels output $w.kernel -p $out | Out-Null
        foreach ($id in $w.ids) {
            $r = Get-ChildItem $out -Recurse -Filter "$id.result" -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($r) { Copy-Item $r.FullName "$q\$id.result" -Force; Remove-Item "$q\$id.req.taken" -ErrorAction SilentlyContinue }
            else { Rename-Item "$q\$id.req.taken" "$id.req" -ErrorAction SilentlyContinue }   # not reached: requeue
        }
        "$(Get-Date -Format HH:mm) $($w.kernel) $($st.Trim() -replace '.*status ','') -> $($w.ids.Count) results"
        $busy.Remove($key)
    }
    # 2. pending requests -> free workers
    $pending = @(Get-ChildItem $q -Filter *.req | Sort-Object LastWriteTime)
    foreach ($tf in $TokenFiles) {
        for ($i = 1; $i -le $WorkersPerAccount -and $pending.Count -gt 0; $i++) {
            $key = "$tf|$i"
            if ($busy.ContainsKey($key)) { continue }
            $batch = @($pending | Select-Object -First $PerBatch)
            $pending = @($pending | Select-Object -Skip $PerBatch)
            $user = UseToken $tf
            $work = Join-Path $env:TEMP "gateq_w_${user}_$i"
            Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
            New-Item -ItemType Directory -Force "$work\data", "$work\kernel" | Out-Null
            $ids = @(); $manifest = @()
            foreach ($req in $batch) {
                $id = $req.BaseName
                $branch, $sha, $repo = (Get-Content $req.FullName -Raw).Trim().Split(' ', 3)
                git -C $repo archive --format=tar.gz -o "$work\data\$id.tgz" $sha
                if ($LASTEXITCODE -ne 0) { Set-Content "$q\$id.result" "$branch $sha`nARCHIVE FAILED"; Remove-Item $req.FullName; continue }
                Rename-Item $req.FullName "$id.req.taken"
                $ids += $id; $manifest += "$id $branch $sha"
            }
            if ($ids.Count -eq 0) { continue }
            Set-Content "$work\data\manifest.txt" ($manifest -join "`n")
            $ds = "$user/srgb-src-$i"
            @{ title = "srgb-src-$i"; id = $ds; licenses = @(@{ name = "CC0-1.0" }) } | ConvertTo-Json | Set-Content "$work\data\dataset-metadata.json"
            if ((python -m kaggle datasets status $ds 2>&1 | Out-String) -match 'ready|pending') { python -m kaggle datasets version -p "$work\data" -m "batch" -q | Out-Null }
            else { python -m kaggle datasets create -p "$work\data" -q | Out-Null }
            for ($k = 0; $k -lt 40; $k++) { if ((python -m kaggle datasets status $ds 2>&1 | Out-String) -match 'ready') { break }; Start-Sleep 10 }
            Copy-Item (Join-Path $PSScriptRoot "batch_kernel.py") "$work\kernel\batch_kernel.py"
            $kernel = "$user/srgb-$i"
            @{ id = $kernel; title = "srgb-$i"; code_file = "batch_kernel.py"; language = "python"; kernel_type = "script";
               is_private = $true; enable_gpu = $false; enable_internet = $true; dataset_sources = @($ds);
               competition_sources = @(); kernel_sources = @() } | ConvertTo-Json | Set-Content "$work\kernel\kernel-metadata.json"
            $push = python -m kaggle kernels push -p "$work\kernel" 2>&1 | Out-String
            if ($push -notmatch 'successfully pushed') {
                "$(Get-Date -Format HH:mm) push refused for $kernel ($(($push -split "`n")[0].Trim())); requeueing"
                foreach ($id in $ids) { Rename-Item "$q\$id.req.taken" "$id.req" -ErrorAction SilentlyContinue }
                continue
            }
            $busy[$key] = @{ ids = $ids; kernel = $kernel }
            "$(Get-Date -Format HH:mm) $kernel <- $($ids -join ', ')"
            Start-Sleep 20   # a fresh push reports the previous run's status for a few seconds
        }
    }
    Start-Sleep 30
}
