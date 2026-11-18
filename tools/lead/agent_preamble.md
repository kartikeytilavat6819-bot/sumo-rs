# Rules for every sumo-rs agent (read first)

- Repo: C:\Users\karti\sumo-rs (Rust port of SUMO 1.28.0). Create your worktree:
  `git -C C:\Users\karti\sumo-rs worktree add .claude/worktrees/agent-<ID> -b worktree-agent-<ID> main` (main = 1973fb63 or later).
  If your branch already exists, reuse its worktree and first `git merge origin/main` (fix conflicts faithfully).
  NEVER edit files under C:\Users\karti\sumo-rs\ directly (that is the lead's main checkout) — only inside your
  worktree C:\Users\karti\sumo-rs\.claude\worktrees\agent-<ID>\. Double-check paths before every edit.
  Read AGENTS.md in the repo, but these rules override its build notes.
- C++ oracle (WSL): `~/sumo-pristine/bin/{sumo,netconvert}`, SUMO_HOME=/home/karti/sumo-pristine/lib/python3.12/site-packages/sumo.
  Official source (READ-ONLY): `~/sumo-src/src` or `C:\Users\karti\sumo-src\src`. Never edit it. Do NOT use ~/sumo-trace (contaminated).
- WSL has ~5 GB RAM and the laptop ~1 GB free: if a C++ run is killed, its output is truncated. Always check output
  completeness. WSL is for single small pristine runs only (references, quick checks); NO C++ builds in WSL.
- Builds: ONLY via vm_gate.ps1 (never call the Kaggle scripts yourself). When all 16 VM slots are busy it queues the
  gate on Kaggle too and runs it wherever a slot frees first (same output format; agent_run.sh that uses
  vm_cpp_trace.sh always waits for a VM). Commit first, then run as a background shell with NO pipe:
  `powershell -ExecutionPolicy Bypass -File C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1 -Branch worktree-agent-<ID>`
  It builds + runs run_all, depart variants, duarouter oracle, unit tests (~10-25 min), then your agent_run.sh.
  Rerunning it for the same commit is fine. If a run fails for an infrastructure reason (ICE, cache, ssh), rerun it.
  NEVER ssh into the VMs or delete anything there (it breaks other agents' builds).
  NO LOCAL RUST BUILDS AT ALL (user's order). Never run cargo on this machine; any cargo process is killed.
  * To run YOUR OWN commands on the VM (traces, single cases, diffs), commit an executable script
    `tools/kaggle/agent_run.sh` on your branch, then run vm_gate.ps1. After the gate it runs from the repo root
    (timeout 30 min) and the last 400 lines of its output are appended to your result after "GATE DONE".
    In it: our binaries are $CARGO_TARGET_DIR/debug/{sumo,netconvert,duarouter}; official C++ 1.28.0 is
    $SUMO_PRISTINE_BIN/{sumo,netconvert} (uninstrumented); SUMO_HOME is set. Print only what you need (grep/diff/head).
  * Instrumented C++ traces: ONLY on the VM, from agent_run.sh:
    `bash tools/gcloud/vm_cpp_trace.sh <ID> <patch-file> [sumo|netconvert]` prints the path of an instrumented binary
    (patch = `git diff` against the official src/ tree, paths src/...). Never use it for references.
  * Remove agent_run.sh (and any trace code) in your final commit: it must never reach main.
- Do NOT run gates on `main`. Main's numbers (eb1c4aa8, gated 2026-11-18): run_all PASS=1228 FAIL=11 SKIP=462,
  depart 15/15, duarouter oracle 55/55, netopts_new 55/55, dfrouter 6/6, activitygen 5/5, od2trips 8/8, marouter 7/7, jtrrouter 10/10, netgenerate 11/11, polyconvert 9/9, unit tests ok. Known failing rows (each has an owner; NOT
  your regressions unless they are your target): out_bt/ref out_meandata/ref elechybrid_overhead/ref md_midinterval/ref md_stop/ref md_vtypes/ref, and 5 newly imported official tests tt_basic_personFlow_walk_routeDist2/ref tt_basic_person_person_inserted_departPos/ref tt_basic_person_walking_walk_routeDist/ref tt_basic_routes_parsing_nested_ref/ref tt_basic_routes_parsing_repeat_stops_no_period/ref.
  run_all has a per-case sim timeout (SIM_TIMEOUT, default 600 s).
- New case references: put an empty NEW_CASE file in the case dir, run tools/oracle/make_sim_refs.sh <case> (it then
  writes tripinfo+fcd+stat, all outputs run_all compares), delete NEW_CASE before committing. Merge main into your branch often.
  Any other failing row on your branch is a regression you must fix.
- TraCI harness (tools/oracle/cases/traci_basic/client.py) uses port+i per probe: use the base port given to you.
- Method: MEASURE first (env-gated trace on both sides, find the first differing quantity), then fix faithfully to the
  official C++. No sorting/tweaking to make one case pass. Pointer-ordered C++ containers: use id/creation order
  with a `// determinism:` comment. Remove all trace code before committing.
- Recurring bug classes (check first): missing virtual dispatch (`DEFAULT_*_VT` fallthrough / base no-op);
  `std::sort` tie order (use `utils::common::std_sort`, SORT_AUDIT.md); base-offset casts from a non-first base
  (use the `*_of` helpers, UB_AUDIT.md); `Box::leak` skipping C++ destructor side effects.
- Forbidden: todo!(), weakening tests, editing refs to match ours (refs are regenerated only from pristine).
- Commit on your branch, never push. Final report < 100 words: measured cause, fix, gate result, sha.
