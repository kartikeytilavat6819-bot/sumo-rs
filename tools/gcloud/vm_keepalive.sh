#!/bin/bash
# Keep both gate VMs running (user's order: VMs must never stop). Spot VMs get preempted -> TERMINATED; restart them.
#   bash tools/gcloud/vm_keepalive.sh      (run in background; checks every 5 min)
export CLOUDSDK_CORE_ACCOUNT=kartikeytilavat6819@gmail.com CLOUDSDK_CORE_PROJECT=project-b17b0335-3081-4c61-89b
while true; do
  for vm in "sumo-gate us-central1-a" "sumo-gate2 us-central1-b"; do
    set -- $vm
    st=$(gcloud compute instances describe "$1" --zone "$2" --format='value(status)' 2>/dev/null)
    if [ "$st" = TERMINATED ] || [ "$st" = STOPPED ]; then
      echo "$(date +%H:%M) $1 was $st: starting"
      gcloud compute instances start "$1" --zone "$2" --async >/dev/null 2>&1
    fi
  done
  sleep 300
done
