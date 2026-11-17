# Option variants: sim comparison against C++ 1.28.0 (pristine)

Each variant is `sim_<v>.args` in `cases/<case>/`, run with the same flags on the C++ binary and on ours, with `-n ref_simple.net.xml -r routes.rou`.
Re-run (Nov 16): C++ = `~/sumo-pristine/bin/sumo`, a fresh run per variant (not the committed `ref_<v>.*` files). Ours: `sumo-rs-target-x07\debug\sumo.exe` (not rebuilt; file dated 14:20 today).
Compared: tripinfo, fcd (`compare_sim.py --tripinfo/--fcd`) and stat (`compare_stat.py`).
Runner: `%TEMP%\opencode\rerun_sim_rv01.sh`; outputs in `%TEMP%\opencode\simopts-rv01\`.

Totals: **PASS=18 FAIL=0 PANIC=0** (18 variants). Old run (`~/sumo-src` as C++ side): PASS=15 FAIL=0 PANIC=3.

| variant | verdict | first diff / panic site | changed vs old run |
|---|---|---|---|
| real/step05 (`--step-length 0.5`) | PASS | — | no |
| real/step01 (`--step-length 0.1`) | PASS | — | no |
| real/ballistic (`--step-method.ballistic true`) | PASS | — | no |
| real/actstep (`--step-length 0.5 --default.action-step-length 1`) | PASS | — | no |
| real/speeddev (`--default.speeddev 0.1 --seed 3`) | PASS | — | no |
| signal/step05 | PASS | — | no |
| signal/step01 | PASS | — | no |
| signal/ballistic | PASS | — | no |
| signal/actstep | PASS | — | no |
| stress/ttt_m1 (`--time-to-teleport -1`) | PASS | — | no |
| stress/ttt30 (`--time-to-teleport 30`) | PASS | — | no |
| stress/mdd5 (`--max-depart-delay 5`) | PASS | — | no |
| stress/collision (`--collision.action warn --collision.check-junctions true`) | PASS | — | no |
| lanechange/lcdur3 (`--lanechange.duration 3`) | PASS | — | no |
| lanechange/lat04 (`--lateral-resolution 0.4`) | PASS | — see note 1 | no |
| reroute_alt/astar (`--routing-algorithm astar`) | PASS | — | **yes: was PANIC stack overflow** |
| reroute_alt/ch (`--routing-algorithm CH`) | PASS | — | **yes: was PANIC "routing algorithm 'CH' is not ported"** |
| reroute_alt/chwrapper (`--routing-algorithm CHWrapper`) | PASS | — | **yes: was PANIC "routing algorithm 'CHWrapper' is not ported"** |

## Notes

1. **Not measured: SL2015 selection by name.** There is no global option for the lane-change model; it is a vType attribute, and the case's `routes.rou` is fixed. The `lanechange/lat04` row covers the path C++ forces when `--lateral-resolution > 0` (`MSAbstractLaneChangeModel.cpp`).
2. **C++ references are fresh runs, not committed files.** Committed `ref_*` files were not touched. The re-run did not depend on them.
3. **Interrupted and killed C++ runs.** In the first pass, `ttt30`, `collision` and `chwrapper` printed `Interrupt signal received` (truncated output) and were rerun; the rerun completed cleanly for all three. In the second pass, `signal/step05` was killed (exit 143); it was rerun alone and completed (exit 0, all closing tags present). Final C++ outputs are complete for all 18 variants. The source of the interrupt/kill is not identified.
4. **Verdict flips (astar, CH, CHWrapper).** These now run and match C++ exactly on tripinfo, fcd and stat. The old panics were probably from an earlier `sumo.exe` build (the old run's build date is not recorded), but this is not verified. Which commit made the change was not checked. Before relying on it, confirm which build produced each side.
5. **Not re-run:** the regression rows from the old loop (`reroute/closing`, `reroute/device`, `reroute_contention/adapt`, `sigma_seed/seed7`) and `real/reroute`. Their verdicts are carried over unchanged from the old run.

Reproduce: `wsl -e bash /mnt/c/Users/karti/AppData/Local/Temp/opencode/rerun_sim_rv01.sh` (run from the worktree root; the script `cd`s there because reroute_alt args use repo-relative paths).
