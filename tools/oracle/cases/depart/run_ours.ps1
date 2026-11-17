# Runs the prebuilt Rust sumo.exe on every depart variant (same args as run_ref.sh).
Set-Location $PSScriptRoot
foreach ($r in Get-ChildItem *.rou) {
  $n = $r.BaseName
  New-Item -ItemType Directory -Force "ours\$n" | Out-Null
  & 'C:\Users\karti\sumo-rs-target-main\debug\sumo.exe' -n '..\junction\ref_simple.net.xml' -r $r.Name `
    --tripinfo-output "ours\$n\tripinfo.xml" --statistic-output "ours\$n\statistic.xml" `
    --seed 42 --end 120 --no-step-log > "ours\$n\stdout.txt" 2> "ours\$n\stderr.txt"
  "$n ours_exit=$LASTEXITCODE"
}
