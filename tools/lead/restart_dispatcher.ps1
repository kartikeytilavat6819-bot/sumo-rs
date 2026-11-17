# One-off: stop the in-memory dispatcher, rebuild its in-flight batches from its log into _busy.json, restart it.
$q = "$HOME\.kaggle\gateq"
$log = "C:\Users\karti\AppData\Local\Temp\opencode\dispatcher2.log"
Get-CimInstance Win32_Process -Filter "Name='powershell.exe'" |
    Where-Object { $_.CommandLine -match 'gate_dispatcher\.ps1' -and $_.CommandLine -notmatch 'Get-CimInstance|restart_dispatcher' } |
    ForEach-Object { taskkill /PID $_.ProcessId /T /F *> $null; "stopped dispatcher $($_.ProcessId)" }
$userTf = @{}
foreach ($tf in @("$HOME\.kaggle\gate_token", "$HOME\.kaggle\gate_token_acct3")) {
    $env:KAGGLE_API_TOKEN = (Get-Content $tf -Raw).Trim()
    $u = ((python -m kaggle config view 2>&1 | Select-String 'username') -split ':')[-1].Trim()
    $userTf[$u] = $tf
}
$inflight = [ordered]@{}
foreach ($l in Get-Content $log) {
    if ($l -match '^\d\d:\d\d (\S+)/srgb-(\d) <- (.*)$') { $inflight["$($Matches[1])/srgb-$($Matches[2])"] = @($Matches[3] -split ', ') }
    elseif ($l -match '^\d\d:\d\d (\S+/srgb-\d) "') { $inflight.Remove($Matches[1]) }
}
$busy = @{}
foreach ($k in $inflight.Keys) {
    $user, $name = $k.Split('/'); $i = $name.Split('-')[1]
    $busy["$($userTf[$user])|$i"] = @{ ids = $inflight[$k]; kernel = $k }
    "in flight: $k <- $($inflight[$k] -join ', ')"
}
$busy | ConvertTo-Json -Depth 4 | Set-Content "$q\_busy.json"
# requests taken by earlier dispatcher runs that nobody tracks any more: drop (their agents re-gate newer commits)
$tracked = $busy.Values | ForEach-Object { $_.ids }
Get-ChildItem $q -Filter '*.req.taken' | Where-Object { $tracked -notcontains ($_.Name -replace '\.req\.taken$', '') } |
    ForEach-Object {
        if ($_.LastWriteTime -lt (Get-Date).AddHours(-1)) { "stale orphan dropped: $($_.Name)"; Remove-Item $_.FullName }
        else { "orphan requeued: $($_.Name)"; Rename-Item $_.FullName ($_.Name -replace '\.taken$', '') }
    }
