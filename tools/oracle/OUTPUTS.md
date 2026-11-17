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
| collision | — | — | PASS (vacuous) | — |
| lanechange | — | — | — | PASS |

**32 rows: PASS 32, FAIL 0, PANIC 0.** Collision is the one vacuous row: the stress case never collides, so both
sides write an empty `<collisions/>` and the match proves nothing about collision output.

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

## Not measured

- **stop-output**: no case in `run_outputs.sh` writes it. `MSStopOut::generateOutputForUnfinished` is a shortcut in
  `ms_net.rs` (`close_simulation`), so unfinished-stop output is unverified.
- **emission-output / edgeData emissions**: owned by em02.
- **collision with real collisions**: needs a case that produces a collision.
- **queue `finish` at close**: `MSQueueExport::finish` is not called from `close_simulation` (shortcut in `ms_net.rs`).
  It only matters with `queue-output.aggregation > 0` and no case covers that.

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
