# Output-file oracle — non-tripinfo outputs

The scoreboard (`SCOREBOARD.md`) compares only tripinfo, FCD and statistics. This page covers the other
outputs users rely on.

Measured 2026-11-16 with `tools/oracle/run_outputs.sh` on the VM gate (`vm_gate.ps1`, branch `worktree-agent-ou03`,
commit `552f5a8e`). Reference = `cases/<case>/ref_out/<output>.xml` (committed, generated from C++ sumo 1.28.0 in
`~/sumo-pristine`). Candidate = our `sumo` debug build (`BIN`), same `sim.args` as `run_all.sh`.

Comparator: `tools/oracle/compare_xml.py REF CAND`. Element by element, in document order. Exact string
compare on attributes and text. Comments (generator header, config echo) are dropped. `<performance>`
elements and the wall-clock `@duration` on `<step>` (summary) are ignored. No numeric tolerance unless `--tol`.

## Results

| output | real | signal | stress | lanechange |
|---|---|---|---|---|
| summary | PASS | PASS | PASS | — |
| queue | PASS | PASS | PASS | — |
| netstate | PASS | PASS | PASS | — |
| vehroute | PASS | PASS | PASS | — |
| vehroute_exit | PASS | PASS | PASS | — |
| edgedata | PASS | PASS | PASS | — |
| full | PASS | PASS | PASS | — |
| amitran | PASS | PASS | PASS | — |
| link | PASS | PASS | PASS | — |
| person | PASS | PASS | PASS | — |
| collision | — | — | PASS (forced: 47 collisions) | — |
| lanechange | — | — | — | PASS |

**32 rows: PASS 32, FAIL 0, PANIC 0.** Collision was vacuous (the stress case never collides); it now runs with forced
collisions, see the ou04 section below.

## Changes that made these pass (this round)

Measured before the fixes on the same harness (VM gate, our build at `c0707ba0`): `edgedata` PANIC (3 rows) and
`person` PANIC (3 rows). Every other row already matched.

- **edgedata**: `<edgeData>` objects are `MSMeanData_Net` → `MSMeanData` → `MSDetectorFileOutput`. `MSMeanData::new` set
  only its own `meandata_vt`, so the base `MSDetectorFileOutput` virtuals (`writeXMLOutput`, `writeXMLDetectorProlog`,
  `detectorUpdate`) still pointed at the pure-virtual defaults. Fixed with a `MSDetectorFileOutputVt` for `MSMeanData`
  (`ms_mean_data.rs`), installed in the constructor.
- **person**: `MSPModel::getActiveNumber` was pure virtual on the base; `MSPModel` had no dispatch table. Added
  `MSPModel.vt` (`MSPModelVt { get_active_number }`), installed by `MSPModel_Interacting` (inherited by Striping and
  JuPedSim, as in C++) and by `MSPModel_NonInteracting`.

Earlier measurements in this file were on a stale x07 binary and are superseded. Summary, queue, netstate, vehroute,
full, amitran, link and lanechange already matched on current main (the old "silent empty output" and link bool
format findings no longer reproduce).

## Round ou04 (VM gate, branch worktree-agent-ou04)

Measured with `tools/kaggle/agent_run.sh` (`run_outputs.sh stops_transport stress`), `BIN=$CARGO_TARGET_DIR/debug/sumo`.

| output | case | verdict | what it proves |
|---|---|---|---|
| stopout | stops_transport | PASS | bus stop, container stop, parking-area and lane stops match C++ (5 `stopinfo`) |
| stopout_unfinished | stops_transport | PASS | `--stop-output.write-unfinished` writes the still-stopped vehicle at close (6 `stopinfo`) |
| collision | stress | PASS | `--collision.mingap-factor 10` forces 47 collisions in C++; ours matches |
| queue_agg | stress | PASS | `--queue-output.aggregation 50`: 4 intervals, last one (150–200) written by `MSQueueExport::finish` at close |

The stress rows (summary, queue, netstate, vehroute, vehroute_exit, edgedata, full, amitran, link, person) also PASS.
The aggregated queue row was not measured before the `finish` call was added, so its PASS shows the final interval
comes out right now, not that the old code was wrong.

Harness notes:
- `stops_transport` is a new case: bus stop `bs0`, container stop `cs0`, parking area `pa0`, one lane stop still
  active at `--end 200`. References come from pristine C++ in WSL.
- `--collision.check-junctions` makes pristine C++ exit 1 with `Invalid Number Format (double) 200`, even as a bare
  flag, so it is not used.
- `stress/sim.args` has CRLF endings. The VM gate converts `*.args` to LF. A local WSL reference run must strip the CR.
- Still open in `MSNet::closeSimulation`: `MSChargingStationExport::write(..., true)` is now called, but no case
  covers `chargingstations-output.aggregated`.

## Round om01: city scale (VM gate, branch worktree-agent-om01)

Runner: `tools/oracle/run_outputs_city.sh` (sibling of run_outputs.sh). No committed references: on the VM both the
official C++ sumo (`$SUMO_PRISTINE_BIN/sumo`) and ours run live with the case's `sim.args`, then `compare_xml.py`
(exact). Each PASS row prints the reference element count, so an empty-output PASS would show up.

Cases: the six largest by vehicle count (reference tripinfo): kumbakonam_mixed (366), autobahn_interchange (282),
zurich_tram (278), karlsruhe_tram (269), cairo_priority (267), hongkong_lefthand_grid (265). Named-city cases in the
113–150 vehicle range (atlanta, paris_dense, milton_keynes, la_interchange, tokyo_lefthand, manhattan_midtown) are
not the largest and were not used.

| output | window | flags |
|---|---|---|
| summary | full | `--summary-output` |
| queue | full | `--queue-output` |
| vehroute / vehroute_exit | full | `--vehroute-output` (+ `--vehroute-output.exit-times true`) |
| edgedata | full | additional `<edgeData freq="60">` file |
| lanechange | full | `--lanechange-output` |
| statistic | full | `--statistic-output --duration-log.statistics true` |
| netstate | 120 s | `--netstate-dump` |
| emission | 120 s | `--emission-output` |
| collision | full | `--collision-output --collision.action warn --collision.mingap-factor 10` (forces collisions) |

Result: **60 rows, PASS 60, FAIL 0, PANIC 0**. Reference element counts (min–max over the six cities): summary
611, queue 1301–6295, vehroute 541–743, edgedata 1110–9580, lanechange 12–216 (kumbakonam 130; zurich 12),
netstate 5167–14411, emission 1898–4970, collision 78–551. Each run took 0–5 s on the VM.

No DIFF was found, so no fix was needed in `src/microsim/output` or devices. Gate counts were unchanged from
main: run_all PASS=301 FAIL=6 SKIP=41 (the known six FAILs), depart 15/15, unit tests ok.

Not covered: the other output types (full, amitran, link, person, stop, chargingstations) at city scale.

## Not measured

- **emission-output / edgeData emissions**: owned by em02.
- **chargingstations-output (aggregated write-unfinished)**: no case writes it.

## Harness notes (not product bugs)

- **C++ reference flake.** Some C++ runs were interrupted ("Interrupt signal received") and wrote a truncated file.
  The driver retries up to 3 times; a row whose C++ run stays bad is marked REFERR. With `REFS=committed` (default)
  no C++ run happens and this cannot occur.
- **SUMO_HOME warning.** Our `sumo` prints `SUMO_HOME is not set properly, disabling XML validation`. That only disables
  validation of the output and does not change any output compared here.

## Reproduce

```bash
bash tools/oracle/run_outputs.sh                 # real signal stress lanechange (committed references)
bash tools/oracle/run_outputs.sh lanechange      # one case
REFS=gen bash tools/oracle/run_outputs.sh        # regenerate references from C++ sumo 1.28.0 (WSL)
python tools/oracle/compare_xml.py <ref.xml> <cand.xml>   # exit 0 MATCH, 1 DIFF, 2 unreadable
```
