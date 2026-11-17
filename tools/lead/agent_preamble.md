# Rules for every sumo-rs agent (read first)

- Repo: C:\Users\karti\sumo-rs (Rust port of SUMO 1.28.0). Create your worktree:
  `git -C C:\Users\karti\sumo-rs worktree add .claude/worktrees/agent-<ID> -b worktree-agent-<ID> main` (main = 1820f92c or later).
  If your branch already exists, reuse its worktree and first `git merge origin/main` (fix conflicts faithfully).
  NEVER edit files under C:\Users\karti\sumo-rs\ directly (that is the lead's main checkout) — only inside your
  worktree C:\Users\karti\sumo-rs\.claude\worktrees\agent-<ID>\. Double-check paths before every edit.
  Read AGENTS.md in the repo, but these rules override its build notes.
- C++ oracle (WSL): `~/sumo-pristine/bin/{sumo,netconvert}`, SUMO_HOME=/home/karti/sumo-pristine/lib/python3.12/site-packages/sumo.
  Official source (READ-ONLY): `~/sumo-src/src` or `C:\Users\karti\sumo-src\src`. Never edit it. For C++ instrumentation,
  copy the source to `~/sumo-<ID>` and build there. Do NOT use ~/sumo-trace (contaminated).
- WSL has ~5 GB RAM: if a C++ run is killed, its output is truncated. Always check output completeness.
- Builds: preferred = Kaggle gate:
  `powershell -ExecutionPolicy Bypass -File C:\Users\karti\sumo-rs\tools\kaggle\gate.ps1 -Branch worktree-agent-<ID>`
  (commit first; it builds + runs run_all, depart, unit tests).
  GOOGLE VM GATE (enabled, faster, use it first): `powershell -ExecutionPolicy Bypass -File
  C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1 -Branch worktree-agent-<ID>` (~2-5 min warm, same output as Kaggle,
  also runs agent_run.sh). USER'S ORDER: use ONLY the VM gate, never the Kaggle gate.ps1 (it is too slow). If a VM
  run fails for an infrastructure reason (ICE, cache, ssh), just rerun vm_gate.ps1.
  NEVER ssh into the VMs or delete anything there (it breaks other agents' builds); the gate retries cache errors itself.
  NO LOCAL RUST BUILDS AT ALL (user's order; the laptop ran out of RAM). Never run cargo on this machine; any cargo
  process is killed. All Rust building/running happens on Kaggle:
  * To run YOUR OWN commands there (traces, single cases, diffs), commit an executable script
    `tools/kaggle/agent_run.sh` on your branch, then run gate.ps1. After the gate, Kaggle runs it from the repo root
    (timeout 25 min) and the last 400 lines of its output are appended to your result after "GATE DONE".
    In it: our binaries are $CARGO_TARGET_DIR/debug/{sumo,netconvert} (/tmp/target on Kaggle); official C++ 1.28.0 is $SUMO_PRISTINE_BIN/{sumo,netconvert}
    (uninstrumented); SUMO_HOME is set. Print only what you need (grep/diff/head), not whole files.
  * Run gate.ps1 as a background shell with NO pipe (never `| Select-Object -First N`): it must stay alive until the
    result arrives, because its completion is what wakes you. Rerunning it for the same commit reattaches; it never re-queues.
  * Remove agent_run.sh (and any trace code) in your final commit: it must never reach main.
  * Instrumented C++ traces: build in WSL (~/sumo-<ID>, make -j2, only the target you need) - that is C++, allowed.
- Do NOT run gates on `main`. Main's numbers (e2a0095c or later): run_all PASS=429 FAIL=7 SKIP=72, duarouter oracle 18/18, duarouter oracle 16/16, depart 15/15, unit
  tests ok. Known FAILs (new cases, each has an owner): 
  city_phoenix_scale_grid PANIC [px01], city_delft_campus_crossings + city_grenoble_tram_bus [cd01], lc_opposite PANIC [lo01], tls_nema_loops (w0 arrival) [nm02], city_rotterdam_rail_yard [rr01], city_seattle_downtown_hills PANIC [sw01]. run_all has a per-case sim timeout (SIM_TIMEOUT, default 600 s).
  These are NOT your regressions: never investigate them unless they are your target.
- New case references: put an empty NEW_CASE file in the case dir, run tools/oracle/make_sim_refs.sh <case> (it then
  writes tripinfo+fcd+stat, all outputs run_all compares), delete NEW_CASE before committing. If you merged main before 6c696d3d and see calibrator
  FAIL, merge main again. Merge main into your branch often.
  Any other failing row on your branch is a regression you must fix.
- TraCI harness (tools/oracle/cases/traci_basic/client.py) uses port+i per probe: use the base port given to you.
- Method: MEASURE first (env-gated trace on both sides, find the first differing quantity), then fix faithfully to the
  official C++. No sorting/tweaking to make one case pass. Pointer-ordered C++ containers: use id/creation order
  with a `// determinism:` comment. Remove all trace code before committing.
- Forbidden: todo!(), weakening tests, editing refs to match ours (refs are regenerated only from pristine).
- Commit on your branch, never push. Final report < 100 words: measured cause, fix, gate result, sha.
