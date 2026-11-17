# Rules for every sumo-rs agent (read first)

- Repo: C:\Users\karti\sumo-rs (Rust port of SUMO 1.28.0). Create your worktree:
  `git -C C:\Users\karti\sumo-rs worktree add .claude/worktrees/agent-<ID> -b worktree-agent-<ID> main` (main = 1820f92c or later).
  If your branch already exists, reuse its worktree and first `git merge origin/main` (fix conflicts faithfully).
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
  also runs agent_run.sh). If it errors or hangs >30 min, fall back to the Kaggle gate.ps1 below.
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
- Do NOT run gates on `main`. Main's numbers: run_all PASS=151 FAIL=5 SKIP=18 (sim FAIL: city_kansas_rural,
  city_karlsruhe_tram, city_london_lh, city_paris_dense, city_tokyo_lefthand), depart 15/15, unit tests ok.
  Any other failing row on your branch is a regression you must fix.
- TraCI harness (tools/oracle/cases/traci_basic/client.py) uses port+i per probe: use the base port given to you.
- Method: MEASURE first (env-gated trace on both sides, find the first differing quantity), then fix faithfully to the
  official C++. No sorting/tweaking to make one case pass. Pointer-ordered C++ containers: use id/creation order
  with a `// determinism:` comment. Remove all trace code before committing.
- Forbidden: todo!(), weakening tests, editing refs to match ours (refs are regenerated only from pristine).
- Commit on your branch, never push. Final report < 100 words: measured cause, fix, gate result, sha.
