#!/bin/bash
# Unattended overnight run of the local model (no Claude account needed):
#   1. keep the laptop awake   2. make sure vLLM is installed (install/retry if not)
#   3. start the vLLM server (restart on crash; smaller context; fall back to Ollama qwen3:8b)
#   4. fill short skeleton methods in rounds, merge each round into main (compile-checked) and push
# Log: C:\Users\karti\sumo-rs-local-fill.log      Launch detached (see bottom of PORTING.md).
export MSYS_NO_PATHCONV=1  # stop Git Bash rewriting /mnt/c/... arguments to wsl.exe
LOG=/c/Users/karti/sumo-rs-local-fill.log
WT=/c/Users/karti/sumo-rs-local
REPO=/c/Users/karti/sumo-rs
STATE=/c/Users/karti/sumo-rs-local-fill.state
# Files Haiku agents are actively working on (skip to avoid merge conflicts)
EXCLUDE=ms_lane.rs,ms_vehicle.rs,nb_edge.rs,nb_node.rs,ni_importer_open_street_map.rs,ms_lane_changer_sublane.rs,ms_parking_area.rs,tra_ci_server.rs,tra_ci_defs.rs,person.rs,ro_edge.rs,ro_net.rs,ro_loader.rs,ro_net_handler.rs,ms_delay_based_traffic_light_logic.rs,ms_push_button.rs,ms_swarm_traffic_light_logic.rs,ms_deterministic_hi_level_traffic_light_logic.rs,nb_traffic_light_definition.rs,nb_own_tl_def.rs,nb_traffic_light_logic.rs,nb_request.rs,nb_edge_cont.rs,nb_node_cont.rs,nb_algorithms.rs,nb_algorithms_ramps.rs,nb_cont_helper.rs,nb_district.rs,nb_district_cont.rs,ms_stage.rs,ms_stage_walking.rs,ms_transportable.rs,nixml_nodes_handler.rs,nixml_edges_handler.rs,nixml_connections_handler.rs,nixml_traffic_lights_handler.rs,nixmlpt_handler.rs

log() { echo "$(date '+%F %T') $*" >> "$LOG"; }
in_wsl() { wsl.exe -d Ubuntu-24.04 -e bash -c "$1"; }

# 1. keep the system awake while this script runs (released automatically when it exits)
powershell.exe -NoProfile -Command '$t=Add-Type -MemberDefinition "[DllImport(\"kernel32.dll\")] public static extern uint SetThreadExecutionState(uint f);" -Name W -Namespace K -PassThru; while($true){ $t::SetThreadExecutionState(0x80000001) | Out-Null; Start-Sleep 30 }' &
AWAKE=$!
trap 'kill $AWAKE $SERVER 2>/dev/null' EXIT
log "start (keep-awake pid $AWAKE)"

# 2. make sure vLLM is importable; if no pip is running and it's still missing, (re)install it
attempt=0
while [ $attempt -lt 4 ]; do
  if in_wsl 'source ~/vllm-env/bin/activate 2>/dev/null && python -c "import vllm" 2>/dev/null'; then log "vllm importable"; break; fi
  if in_wsl 'pgrep -f "pip install" >/dev/null'; then log "pip install still running, waiting"; sleep 120; continue; fi
  attempt=$((attempt+1))
  log "vllm missing, installing (attempt $attempt)"
  in_wsl 'python3 -m venv ~/vllm-env 2>/dev/null; source ~/vllm-env/bin/activate && pip install -q vllm' >> "$LOG" 2>&1
done

SERVER=""
API_URL=""
api_up() { curl -s --max-time 5 "$1/models" | grep -q '"id"'; }
start_vllm() {  # $1 = max model len; returns 0 when the API answers
  log "starting vLLM (max-model-len $1)"
  wsl.exe -d Ubuntu-24.04 -e bash /mnt/c/Users/karti/sumo-rs/tools/start_vllm.sh "$1" >> /c/Users/karti/vllm-server.log 2>&1 &
  SERVER=$!
  for i in $(seq 1 120); do   # up to 60 min: first start downloads the model (~5.5 GB)
    api_up http://localhost:8000/v1 && return 0
    kill -0 "$SERVER" 2>/dev/null || { log "vLLM exited (see vllm-server.log)"; return 1; }
    sleep 30
  done
  return 1
}
ensure_server() {
  [ -n "$API_URL" ] && api_up "$API_URL" && return 0
  kill "$SERVER" 2>/dev/null; in_wsl 'pkill -f "vllm serve"' 2>/dev/null; sleep 5
  for len in 8192 8192 4096; do
    if start_vllm $len; then API_URL=http://localhost:8000/v1; export LOCAL_LLM_URL=$API_URL/chat/completions LOCAL_LLM_MODEL=""; log "vLLM up"; return 0; fi
    in_wsl 'pkill -f "vllm serve"' 2>/dev/null; sleep 10
  done
  # last resort: the Ollama model already on this laptop
  log "vLLM failed 3 times; falling back to Ollama qwen3:8b"
  (ollama serve >/dev/null 2>&1 &); sleep 10
  if api_up http://localhost:11434/v1; then
    API_URL=http://localhost:11434/v1; export LOCAL_LLM_URL=$API_URL/chat/completions LOCAL_LLM_MODEL=qwen3:8b; return 0
  fi
  log "no local model server available"; return 1
}

# 3. fill in rounds; merge each round into main; keep going until no files are left
cd "$WT" || { log "worktree $WT missing"; exit 1; }
for round in $(seq 1 1000); do
  ensure_server || { sleep 300; continue; }
  git -C "$WT" merge -q --no-edit main >> "$LOG" 2>&1 || git -C "$WT" merge --abort 2>/dev/null
  OUT=$(python "$REPO/tools/local_fill.py" --worktree "$WT" --max-lines 15 --limit-files 15 \
          --state "$STATE" --exclude "$EXCLUDE" 2>&1)
  echo "$OUT" | grep -v '^$' >> "$LOG"
  log "round $round done; merging into main"
  RES=$(bash "$REPO/tools/merge_agent.sh" local-fill 2>&1); echo "$RES" | grep -v '^warning' >> "$LOG"
  if echo "$RES" | grep -qE "build errors|CONFLICT"; then
    # main moved under us: drop this branch's unmerged work and retry those files next round
    for f in $(git -C "$WT" diff --name-only main...local-fill); do
      grep -vxF "$f" "$STATE" > "$STATE.tmp" 2>/dev/null; mv "$STATE.tmp" "$STATE"
    done
    git -C "$WT" reset -q --hard main; git -C "$REPO" branch -D -q fix-local-fill 2>/dev/null
    log "merge failed; reset local-fill to main, files queued again"
  fi
  echo "$OUT" | grep -q "REMAINING_FILES=0" && { log "all files processed"; break; }
done
log "local fill finished; $(python "$REPO/tools/progress.py" | tail -1)"
