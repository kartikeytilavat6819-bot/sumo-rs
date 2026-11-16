# Scoreboard — current state

Main `c599e28` ("Merge branch 'wave2'"), 2026-11-16.

Numbers from the lead's run on main: **`run_all.sh` PASS=71 FAIL=4 SKIP=13; depart 15/15.**
This worktree did not re-run the harness. `C:\Users\karti\sumo-rs-target-x07\debug\sumo.exe` (10:49) is older than
the newest source on main (`crates/sumo_core/src/netbuild/nbpt_line_cont.rs`, 10:50), so `run_all.sh` would print
`STALE BINARY`. Per-case verdicts below use the lead's FAIL list. Where a case has a per-case `REPORT.md`, its
date and verdict are quoted. Those reports can disagree with the lead's run; they are marked as such.

Reproduce (Git Bash, a binary built from the tree under test):

```bash
CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-x07 bash tools/oracle/run_all.sh     # exact comparator; prints STALE BINARY if stale
bash tools/oracle/cases/depart/run_all_variants.sh                                   # 15 depart variants
python tools/oracle/acceptance.py <case> <cand.ti.xml> <cand.stat.xml>               # tolerance: PASS/CLOSE/FAIL
```

`run_all.sh` runs the checkout it lives in. The sim stage runs whenever `ref_simple.net.xml`, `routes.rou` and
`ref.tripinfo.xml` exist, and appends `cases/<case>/sim.args` to the sumo call. The net stage runs only with
`input.osm`, and appends `cases/<case>/net.args` to the netconvert call.

## FAIL rows on main

| row | symptom | cause / owner |
|---|---|---|
| `city_munich` net | 12 junction-classification diffs | agent nc03 |
| `reroute_alt/ref_astar` | stack overflow | fixed on branch `rr04`, not on main |
| `reroute_alt/ref_ch` | shortcut panic | fixed on branch `rr04`, not on main |
| `reroute_alt/ref_chwrapper` | shortcut panic | fixed on branch `rr04`, not on main |

## Case directories (`tools/oracle/cases/`)

"Main" is the lead's run: FAIL only if listed above, otherwise not FAIL. Where an older doc says FAIL, the doc is
quoted, and that row needs a re-measure before anyone trusts it.

| case | what it tests | main | notes |
|---|---|---|---|
| city_munich | central Munich old-town grid, net + sim | FAIL (net) | net: 12 junction-classification diffs (nc03) |
| city_plano | US suburban arterial, net + sim | not FAIL | REPORT (2026-11-16): net 7378/7378 exact, tripinfo 137 entries MATCH, fcd and stat PASS |
| closed_edge | lane closed after departure; `v0` reroutes off it at t=8 | not FAIL | SCOREBOARD says PASS; `REPORT.md` says sim FAIL (`v0` arrival 30.00 ref, 31.00 ours). Re-measure |
| depart | 15 departure variants: `lane_*`, `pos_*`, `speed_*` | 15/15 PASS | Linux gave lane_1 and pos_max FAIL: CRLF in the reference stderr (see Kaggle section). Fix is on `worktree-agent-px01`; not on main |
| detectors | detector output files plus sim | not FAIL | REPORT: sim PASS (tripinfo, fcd, stat); `det_lad.xml` (laneAreaDetector) FAIL; detector files PARTIAL |
| junction | junction net and sim | not FAIL | SCOREBOARD: PASS |
| lanechange | lane change, `lcdur3` and `lat04` variants | not FAIL | REPORT: net exact, sim PASS. Its `<change>` output FAIL (see Outputs) |
| lefthand | left-hand traffic; needs `--lefthand` via `net.args` | not FAIL | sim PASS. Net stage needs the flag |
| netconv_edge | netconvert on bad and unusual OSM | not FAIL | REPORT: 7 nets MATCH, 3 SKIP. Error cases panic: truncated OSM, zero-byte file |
| real | Times Square net and sim | not FAIL | SCOREBOARD: byte-identical. `ref_reroute` PASS (closure hits the destination, so no real detour) |
| real_dense | congested Times Square, `--end 125` | not FAIL | SCOREBOARD (older): FAIL, `v15` arrival 5.29 vs 7.6 (op02). Re-measure |
| reroute | rerouter and device: `ref`, `ref_closing`, `ref_device` | not FAIL | `ref` and `ref_closing` PASS. SCOREBOARD (older): `ref_device` FAIL (`routing_v1` device never created). Re-measure |
| reroute_alt | `v0` reroutes at t=0 to `10 → 12 → 13` | FAIL (`ref_astar`, `ref_ch`, `ref_chwrapper`) | `ref` itself: REPORT says FAIL (30.00 vs 31.00), not in the lead's list. Re-measure |
| reroute_bypass | mid-route detour through a bypass, `period=1000` | not FAIL | REPORT: PASS (`LEAD IN AB OUT` → `LEAD IN AC CB OUT`) |
| reroute_bypass_p1 | same net, `period=1`; C++ reverts the detour | not FAIL per lead | CASE.md says **expected FAIL**. Lead's 4-row list does not include it. Verify before trusting |
| reroute_contention | many vehicles reroute at once and compete for a bypass | not FAIL | REPORT: net, `ref` and `ref_adapt` PASS. SCOREBOARD's "no contention case" gap is stale |
| roundabout | roundabout net and sim | not FAIL | SCOREBOARD: PASS |
| route_check | disconnected route: what C++ does (`gCheckRoutes`) | not FAIL | ignore variant PASS. Default error path PASS after fix, via `run_error.sh` |
| routes_forms | route file syntax forms | not FAIL | no REPORT; no recorded verdict |
| sigma_seed | RNG parity: `sigma` and `speedFactor` draws under `--seed` | not FAIL | REPORT: PASS for both seeds (exit 0, no panic) |
| signal | signal case; t=34 divergence was explicit vs semi-implicit Euler | not FAIL | SCOREBOARD: tripinfo, FCD and statistics all exact |
| signal_actuated | actuated signal control | not FAIL | REPORT: net and sim PASS |
| signal_delay | signal delay | not FAIL | REPORT: net and sim PASS |
| signal_nema | NEMA signal control | not FAIL | REPORT: net and sim PASS |
| stops | stops | not FAIL | no REPORT; no recorded verdict |
| stops_parking | lane stop, `until` stop, parking-area stop, multi-stop | not FAIL | REPORT has a verdict section; read it there |
| stress | 120 vehicles oversaturating Times Square | not FAIL | SCOREBOARD: tripinfo, FCD, stat match exactly |
| sublanes | sublane model | not FAIL | SCOREBOARD: PASS. REPORT (2026-11-16): sim PANIC at `ms_leader_info.rs:503`. Re-measure |
| teleport | blocked jam triggers `--time-to-teleport 20` | not FAIL | REPORT has a verdict section; read it there |
| traci_basic | TraCI command probes | not FAIL | see `tools/oracle/TRAci_STATUS.md`. Probe counts disagree across docs (58/62 and 44/62) |
| trips | trips input | not FAIL | no REPORT; no recorded verdict |
| vtypes | vType and car-following: EIDM, Wiedemann | not FAIL | SCOREBOARD (older): run_all FAIL (`--end 120`); acceptance CLOSE (`EIDM_1` timeLoss 4.05 vs 4.02). Re-measure |
| zipper | 2-lane edge into a zipper merge | not FAIL | REPORT: FAIL, FCD t=19 `z2` x 220.25 ref vs 220.81 ours. Re-measure |

Sim-stage verdicts with no reference (`ref.tripinfo.xml` missing) count as SKIP. The lead's SKIP count is 13.
The lead's run did not give per-row SKIP names.

## Outputs (non-tripinfo)

Full page: `tools/oracle/OUTPUTS.md` on branch `worktree-agent-out01` (not on main). Measured 2026-11-16 against the
x07 debug build, which was not rebuilt. **PASS 4, FAIL 22, PANIC 6** over 32 rows.

- **Silent empty output.** `queue`, `netstate`, `vehroute`, `full` and `amitran` exit 0 and write a header with no
  data elements. No warning.
- **Panics.** `<edgeData>` in `--additional-files` panics in `MSDetectorFileOutput` (pure virtual
  `writeXMLDetectorProlog`). `--person-summary-output` panics in `MSPModel::getActiveNumber` (pure virtual).
- **`lanechange` output.** Ours drops nine gap attributes that C++ writes on every `<change>`.
- **`link` output.** C++ writes `willPass="1"`; ours writes `"true"`.
- **`summary`** passes after ignoring the wall-clock `@duration`.
- **`collision`** is a vacuous PASS: the stress case never collides.

Owners as given by the lead: ou02, dv02, cf02, pd01.

## Known gaps (owner)

| gap | owner |
|---|---|
| Outputs: 4/32 pass (see above) | ou02, dv02, cf02, pd01 |
| battery and devices never built | dv02 |
| OSM import requires `--simple-projection` (UTM) | pj01 |
| car-following: W99 `cc1`, KraussPS, CACC | cf03 |
| `left_before_right` junction | jt01 |
| TraCI | `tools/oracle/TRAci_STATUS.md` |
| `loadState` | st01 |
| meso (`--mesosim`) | ms01 (scoping) |
| rail | rb01 |
| persons | pd01 |
| emissions | em01 |

Meso is a loud error (`mesoscopic simulation is not wired into MSNet yet`, `ms_net.rs:401`), not a silent
fallback. That is correct for a missing subsystem.

## Coverage gaps

Taken from the 2026-11-16 SCOREBOARD. Not re-checked today.

| gap | why it matters | what would cover it |
|---|---|---|
| closed-edge enforcement is never exercised | every closure in our cases is a router prohibition; SUMO never holds a vehicle for it | a case with `disallow="all"` on an edge a vehicle must traverse |
| meso | separate engine | meso port first |
| rail | no rail net in any case | small rail net with `--railsignal-moving-block` |
| TraCI writes | `changeLane`, `moveToXY`, route setting and subscriptions unmeasured | extend `cases/traci_basic/client.py` |
| `depart` references | re-derived on demand only | re-run `run_ref.sh` periodically |

## Kaggle gate

`tools/kaggle/remote_gate.ps1` runs the full oracle gate for one git branch on a Kaggle CPU notebook, so the work
runs off this machine's RAM.

```powershell
powershell -File tools/kaggle/remote_gate.ps1 -Branch worktree-agent-x -Slot 1
```

What it does:

1. `git archive` of the branch, plus a `GATE_BRANCH.txt` marker with the branch name and short sha.
2. Uploads that as a private dataset `<user>/sumo-rs-gate-src-<slot>`. Creates it on first use, then new versions.
3. Pushes a private, internet-enabled CPU kernel `<user>/sumo-rs-gate-<slot>` running `gate_kernel.py`.
4. Polls status up to `-TimeoutMin` (default 90), then fetches the output and prints `gate.txt`.

- **Slots 1 to 5.** Each is a separate dataset and kernel pair, so up to five gates run at once.
- **Token.** `-TokenFile` defaults to `%USERPROFILE%\.kaggle\gate_token` (a Kaggle `KGAT_...` token). The script
  comment names a second file, `gate_token_acct3`, for another account. The script reads the token value into
  `KAGGLE_API_TOKEN`. Do not paste token values anywhere. The `.kaggle` folder, checked from this shell, was empty.
- **Result.** Linux and Windows agree. The only difference found (`tools/oracle/PLATFORM_DIFF.md` on branch
  `worktree-agent-px01`) was CRLF in the depart reference `stderr.txt` files. GNU grep keeps the `\r`; MSYS2 grep
  strips it. The fix is one `tr -d '\r'` on line 26 of `cases/depart/run_all_variants.sh`. It is not on main.

## Branch triage

`tools/oracle/BRANCH_TRIAGE.md` (copied from `worktree-agent-br01`). **0 useful, 12 unknown, 12 superseded.**

- **Never merge `fix-a60ca9556cae1357c`.** It replaces main's real `libsumo/vehicle.rs::add` body with a
  `panic!("... not yet ported")` stub, and would regress about 3,674 spots.
- The other branches are mostly behind main. Merging them would regress main.
- The 12 "unknown" branches add new functions that still need a C++ check. Nothing was merged, deleted or pushed.

## Traps that have cost real time here

- **Stale binary.** Shared target dir gets overwritten by other agents. Build before trusting a run.
- **Stale case reports.** `REPORT.md` files record the binary they were run with. `stress`, `reroute`, `vtypes` and
  `depart` reports describe failures and panics that the current binary does not show. Re-run before acting on them.
- **Harness measured the wrong tree.** `run_all.sh` used to `cd` to the main checkout and a fixed target dir, so a
  worktree's scoreboard measured main's code. It now runs its own checkout; set `CARGO_TARGET_DIR` to the build under test.
- **Silently skipped cases.** A `[ -f input.osm ] || continue` guard hid four cases from the scoreboard. Every case
  now prints a row, including SKIPs with their reason.
- **Stub returning a permissive default.** `g_*()` copied into a file with a hardcoded permissive default
  (`0`/`false`) instead of calling `ms_globals`: no panic, no warning, traffic just flows too freely.
  This was the junction defect.
- **Ported but never called.** `schedule_initial_switch` was a perfect translation that nothing invoked.
- **Reference generated with different flags than the harness runs.** `real_dense`'s reference needs
  `--end 125`; without it the id sets disagree and it looks like an extra vehicle. `make_sim_refs.sh` now reads
  `sim.args`, so the two cannot drift.
