#!/bin/bash
# Lead driver for one OpenCode agent on a FREE model (user's order: OpenCode CLI, free models only).
# OpenCode edits + commits in its own worktree; after each round the lead's VM gate checks the branch and the
# result (with regressions vs main's known FAILs) goes back into the same OpenCode session.
#   bash tools/lead/oc_agent.sh <ID> <task-file> [max-rounds]     (run in background)
# Progress: .claude/oc-<ID>.log (short), .claude/oc-<ID>.log.json (raw OpenCode events), .claude/oc-<ID>.lastgate
set -u
ID=$1 TASK=$2 ROUNDS=${3:-10}
REPO=/c/Users/karti/sumo-rs WT=$REPO/.claude/worktrees/agent-$ID BR=worktree-agent-$ID
OC="$APPDATA/npm/node_modules/@opencode/cli/bin/opencode.exe"
MODEL=${OC_MODEL:-opencode-go/beam-free}
# user (2026-11-18): only beam-free from now on; the opencode/ copy of beam is the fallback when the opencode-go one
# hits its free limit (beam was the fastest correct model in the known-answer benchmark).
MODELS=" $MODEL ${OC_FALLBACKS:-opencode/beam-free} "
next_model() { local rest=${MODELS#* $1 }; [ "$rest" = "$MODELS" ] && rest=$MODELS; set -- $rest; echo "${1:-}"; }
LOG=$REPO/.claude/oc-$ID.log
# rows failing on main 3add06405: not regressions
KNOWN=" out_bt/ref out_meandata/ref elechybrid_overhead/ref md_midinterval/ref md_stop/ref md_vtypes/ref tt_basic_routes_parsing_nested_ref/ref tt_basic_routes_parsing_repeat_stops_no_period/ref tt_bugs_13xxx_ticket13108/ref tt_bugs_17xxx_ticket17583/ref tt_bugs_4xxx_ticket4299/ref itt_nc_netconvert_bugs_5xxx_ticket6497 duarouter_itt_duarouter_meta_write_config itt_nc_netconvert_import_OSM_railway_RE70 itt_nc_netconvert_import_OSM_railway_S5_bidi_stop_turns itt_nc_netconvert_import_OSM_railway_inconsistent_directions itt_nc_netconvert_import_OSM_railway_signal_dir_override_repair tt_junction_model_indirect_left_turn_internal_junction_controlled_moped/ref tt_spec_distribution_route_nested/ref tt_spec_distribution_route_nested_ref/ref tt_sublane_model_junction_indirect_left_moped/ref tt_sublane_model_junction_turnAlignment_moped_indirect_left/ref tt_tls_actuated_conditions_assignments_find_primes/ref tt_tls_actuated_conditions_assignments_find_primes_function/ref tt_tls_actuated_conditions_assignments_function_local_vars/ref tt_tls_actuated_conditions_assignments_function_nesting/ref "

say() { echo "$(date +%H:%M) $*" | tee -a "$LOG"; }
# every failing line whose row is not a known main FAIL, plus build/test/harness failures
regressions() {
  local r=""
  grep -q 'GATE DONE' <<<"$1" || r="gate did not finish;"
  grep -q 'BUILD FAILED' <<<"$1" || grep -q '^PASS=[0-9]' <<<"$1" || r="$r no run_all summary (partial gate);"
  while read -r _line; do
    read -r row _ <<<"$_line"
    if [ "$row" = "netopts_new:" ]; then
      # one summary line lists every failing netconvert row as name:FAIL; judge each row by name
      for nr in $(grep -oE '[A-Za-z0-9_.-]+:(FAIL|PANIC)' <<<"$_line" | cut -d: -f1); do
        [[ "$KNOWN" == *" $nr "* ]] || r="$r $nr;"
      done
      continue
    fi
    [[ "$KNOWN" == *" $row "* ]] || r="$r $row;"
  done < <(grep -E 'FAIL|PANIC|TIMEOUT|panicked|STALE|MISSING' <<<"$1" | grep -vE '(^| )PASS=[0-9]+ FAIL=0|^PASS=')
  # the run_all summary "PASS=n FAIL=m" is judged by the rows above, not by its count
  echo "$r"
}
last_text() { python -c "
import sys, json
t = ''
for l in sys.stdin:
    try: p = (json.loads(l).get('part') or {})
    except Exception: continue
    if p.get('type') == 'text': t = p.get('text', '')
print(t[-1500:])"; }

[ -d "$WT" ] || git -C "$REPO" worktree add -q "$WT" -b "$BR" "${OC_BASE:-main}"
cp "$REPO/tools/lead/oc_permissions.jsonc" "$WT/opencode.jsonc"
msg="$(sed "s/<ID>/$ID/g" "$REPO/tools/lead/${OC_RULES:-oc_rules.md}"; echo; cat "$TASK")"
# OC_BASELINE=1 (branches not based on main): gate the untouched base once; whatever fails there is not a regression
if [ -n "${OC_BASELINE:-}" ]; then
  say "baseline gate of $(git -C "$WT" rev-parse --short HEAD)"
  base=$(powershell.exe -ExecutionPolicy Bypass -File 'C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1' -Branch "$BR" 2>&1 | tr -d '\r')
  KNOWN="$KNOWN $(grep -E 'FAIL|PANIC|TIMEOUT|panicked|STALE|MISSING' <<<"$base" | grep -vE '^PASS=' | awk '{print $1}' | tr '\n' ' ') "
  say "baseline known rows:$KNOWN"
fi
sid=""
for r in $(seq 1 "$ROUNDS"); do
  before=$(git -C "$WT" rev-parse HEAD)
  say "round $r: $MODEL working${sid:+ (session $sid)}"
  out=$(cd "$WT" && printf '%s' "$msg" | timeout 3600 "$OC" run --auto -m "$MODEL" --format json --archived ${sid:+--session "$sid"} 2>&1)
  printf '%s\n' "$out" >> "$LOG.json"
  [ -n "$sid" ] || sid=$(grep -o '"sessionID":"ses_[A-Za-z0-9]*"' <<<"$out" | head -1 | cut -d'"' -f4)
  if grep -q 'Free usage limit reached' <<<"$out"; then
    # switch to the next free model and resend the same message; stop only when every model is limited
    tried="${tried:-} $MODEL"; next=$(next_model "$MODEL")
    if [ -z "$next" ] || [[ " $tried " == *" $next "* ]]; then say "free usage limit reached on every model ($tried): stopping"; break; fi
    say "free usage limit on $MODEL: switching to $next"; MODEL=$next; continue
  fi
  tried=""
  say "reply: $(last_text <<<"$out" | tr '\n' ' ' | cut -c1-600)"
  if [ -n "$(git -C "$WT" status --porcelain)" ]; then
    msg="You left uncommitted changes. Commit them (git add -A && git commit) and reply with your summary."; continue
  fi
  after=$(git -C "$WT" rev-parse HEAD)
  if [ "$before" = "$after" ]; then
    # the free model sometimes ends its turn mid-task ("Let me verify..."): nudge it twice before giving up
    if ! last_text <<<"$out" | grep -q 'DONE' && [ "${nudges:-0}" -lt 2 ]; then
      nudges=$(( ${nudges:-0} + 1 )); say "no commit yet: nudging ($nudges)"
      msg="Continue your task now: do the work (edit files, generate references, commit with git add -A && git commit). Do not stop to announce what you will do next; stop only after a commit, with a short summary, or reply DONE."
      continue
    fi
    say "no new commit: stopping"; break
  fi
  nudges=0
  sha=$(git -C "$WT" rev-parse --short HEAD)
  say "gating $sha"
  res=$(powershell.exe -ExecutionPolicy Bypass -File 'C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1' -Branch "$BR" 2>&1 | tr -d '\r')
  printf '%s\n' "$res" > "$REPO/.claude/oc-$ID.lastgate"
  reg=$(regressions "$res")
  say "gate $sha: $(grep -E '^PASS=|depart variants|duarouter oracle|BUILD FAILED' <<<"$res" | tr '\n' ' ') | regressions: ${reg:-none}"
  msg="Gate result for your commit $sha:
$res

Regressions (rows that pass on main but not on your branch, or build/test failures): ${reg:-none}.
If BUILD FAILED: fix the compile errors shown above (they are in your files). If a previously passing row now fails:
find which of your changes caused it and fix it faithfully, or undo that change. Commit, then continue with the next
10-15 shortcuts in your area and reply with your summary. If nothing more in your area can be ported faithfully, reply DONE with a final summary."
done
say "finished after round $r; branch $BR at $(git -C "$WT" rev-parse --short HEAD)"
