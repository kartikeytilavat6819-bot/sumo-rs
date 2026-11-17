# Gate one branch on the Google Cloud VM (16 cores, warm target dir): no queue, ~2-5 min incremental.
#   powershell -ExecutionPolicy Bypass -File C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1 -Branch wave2
# Paid from the project's welcome credit; the VM stops itself after 30 idle minutes.
param(
    [Parameter(Mandatory = $true)][string]$Branch,
    [string]$Repo = "C:\Users\karti\sumo-rs",
    [string]$Vm = ""
)
$ErrorActionPreference = "Continue"
# two VMs; a branch always goes to the same one so its incremental build stays warm (wave2 = lead -> sumo-gate)
$vms = @(@{ n = "sumo-gate"; z = "us-central1-a" }, @{ n = "sumo-gate2"; z = "us-central1-b" })
if (-not $Vm) { $Vm = if ($Branch -eq "wave2") { "sumo-gate" } else { $vms[([int[]][char[]]$Branch | Measure-Object -Sum).Sum % 2].n } }
$Zone = ($vms | Where-Object { $_.n -eq $Vm }).z
$sha = (git -C $Repo rev-parse --short $Branch).Trim()
if (-not $sha) { "unknown branch $Branch"; exit 1 }
$id = (("$Branch-$sha".ToLower()) -replace '[^a-z0-9]+', '-').Trim('-')
$tgz = Join-Path $env:TEMP "$id.tgz"
git -C $Repo archive --format=tar.gz -o $tgz $sha
$st = gcloud compute instances describe $Vm --zone $Zone --format "value(status)" 2>$null
if ($st -ne "RUNNING") { gcloud compute instances start $Vm --zone $Zone --quiet 2>&1 | Out-Null; Start-Sleep 30 }
$run = Join-Path $PSScriptRoot "vm_gate_run.sh"
gcloud compute scp $tgz $run "${Vm}:/tmp/" --zone $Zone --quiet 2>&1 | Out-Null
gcloud compute ssh $Vm --zone $Zone --quiet --command "while [ ! -f /opt/gate_ready ]; do sleep 10; done; sudo cp /tmp/$id.tgz /home/gate/ && sudo cp /tmp/vm_gate_run.sh /home/gate/run_$id.sh && sudo sed -i 's/\r$//' /home/gate/run_$id.sh && sudo chown gate /home/gate/$id.tgz /home/gate/run_$id.sh && sudo -iu gate bash /home/gate/run_$id.sh /home/gate/$id.tgz '$Branch $sha'; sudo rm -f /home/gate/$id.tgz /home/gate/run_$id.sh" 2>&1 |
    Where-Object { $_ -notmatch '^(Updating|Warning: Permanently|External IP|Writing|.*NativeCommandError|.*CategoryInfo|.*At .*gcloud\.ps1|\s*\+|\s*~)' }
Remove-Item $tgz -ErrorAction SilentlyContinue
