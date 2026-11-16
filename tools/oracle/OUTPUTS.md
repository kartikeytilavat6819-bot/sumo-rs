# Output-file oracle — non-tripinfo outputs

The scoreboard (`SCOREBOARD.md`) compares only tripinfo, FCD and statistics. This page covers the other
outputs users rely on. Measured 2026-11-16 with `tools/oracle/run_outputs.sh`, our `sumo.exe` from
`C:\Users\karti\sumo-rs-target-x07\debug` (not rebuilt), C++ sumo 1.28.0 in WSL, same `sim.args` as `run_all.sh`.

Comparator: `tools/oracle/compare_xml.py REF CAND`. Element by element, in document order. Exact string
compare on attributes and text. Comments (generator header, config echo) are dropped. `<performance>`
elements and the wall-clock `@duration` on `<step>` (summary) are ignored. No numeric tolerance unless `--tol`.

Results table: `C:\Users\karti\Temp\score\outputs\results.tsv` (run 2; scratch, not committed).

| output | case | verdict | first diff / panic |
|---|---|---|---|
| summary | real | PASS | — |
| summary | signal | PASS | — |
| summary | stress | PASS | — |
| queue | real | FAIL | ours writes header only: `/queue-export` ref=24 children, cand=0 |
| queue | signal | FAIL | ours header only: ref=43 children, cand=0 |
| queue | stress | FAIL | ours header only: ref=374 children, cand=0 |
| netstate | real | FAIL | ours header only: ref=24 timesteps, cand=0 |
| netstate | signal | FAIL | ours header only: ref=43, cand=0 |
| netstate | stress | FAIL | ours header only: ref=374, cand=0 |
| vehroute | real | FAIL | ours header only: `/routes` ref=6, cand=0 |
| vehroute | signal | FAIL | ours header only: ref=5, cand=0 |
| vehroute | stress | FAIL | ours header only: ref=120, cand=0 |
| vehroute_exit | real | FAIL | same as vehroute (exit-times adds nothing on our side) |
| vehroute_exit | signal | FAIL | same as vehroute |
| vehroute_exit | stress | FAIL | same as vehroute |
| edgedata | real | PANIC | `ms_detector_file_output.rs:28`: pure virtual `MSDetectorFileOutput::writeXMLDetectorProlog` |
| edgedata | signal | PANIC | same panic |
| edgedata | stress | PANIC | same panic |
| full | real | FAIL | ours header only: `/full-export` ref=24, cand=0 |
| full | signal | FAIL | ours header only: ref=43, cand=0 |
| full | stress | FAIL | ours header only: ref=374, cand=0 |
| amitran | real | FAIL | ours header only: `/trajectories` ref=55, cand=0 |
| amitran | signal | FAIL | ours header only: ref=90, cand=0 |
| amitran | stress | FAIL | ours header only: ref=3904, cand=0 |
| link | real | FAIL | `/link-output/timestep/link/approaching @willPass`: ref `1`, cand `true` (bool format) |
| link | signal | FAIL | same bool-format diff |
| link | stress | FAIL | same bool-format diff |
| person | real | PANIC | `msp_model.rs:152`: `MSPModel::getActiveNumber is pure virtual` |
| person | signal | PANIC | same panic |
| person | stress | PANIC | same panic |
| lanechange | lanechange | FAIL | `/lanechanges/change`: ref has `leaderGap, leaderSecureGap, leaderSpeed, followerGap, followerSecureGap, followerSpeed, origLeaderGap, origLeaderSecureGap, origLeaderSpeed`; ours omits all nine |
| collision | stress | PASS (vacuous) | both write an empty `<collisions/>`: stress has 0 collisions, so this proves nothing about collision output |

Totals: **PASS 4, FAIL 22, PANIC 6, REFERR 0** (32 rows; a REFERR row appeared once and is recorded below).

## Findings to act on

1. **Silent empty output (worst).** queue, netstate, vehroute, full and amitran all exit 0 and write a
   header with no data elements (about 810 bytes). No warning, no panic. A user gets a valid-looking file with nothing in it.
2. **Panics.** `--additional-files` with `<edgeData>` panics in `MSDetectorFileOutput` (pure virtual
   `writeXMLDetectorProlog`). `--person-summary-output` panics in `MSPModel::getActiveNumber` (pure virtual).
3. **lanechange-output content.** Ours drops nine gap attributes that C++ writes on every `<change>`.
4. **link-output bool format.** C++ writes `willPass="1"`, ours writes `"true"`.
5. **summary: passes** after ignoring wall-clock `@duration`. Before that, the only difference was that field.
6. **collision: no real test.** The stress case never collides. Needs a case that produces a collision.

## Harness notes (not product bugs)

- **C++ reference flake.** Some C++ runs were interrupted ("Interrupt signal received, trying to exit
  gracefully") and wrote a truncated file. Adding `</dev/null` to the `wsl` call reduced it but did not remove it:
  several rows still needed a second attempt. The cause is unknown. The driver retries up to 3 times, and a row
  whose C++ run stays bad is marked **REFERR**, not PASS/FAIL.
  `stress/full` was REFERR in one run (all 3 attempts interrupted) and FAIL with a valid reference in the next.
  Treat that row as unresolved until it reruns clean.
- **Run-to-run stability.** Two full runs gave identical verdicts and child counts; the only difference was
  the thread id inside panic messages.
- **SUMO_HOME warning.** Our `sumo.exe` prints `SUMO_HOME is not set properly, disabling XML validation`.
  That only disables validation of the output and does not change any output compared here.
- Our binary was not rebuilt; this measures the x07 debug build of `2026-11-16`.

## Reproduce

```bash
bash tools/oracle/run_outputs.sh                 # real signal stress lanechange
bash tools/oracle/run_outputs.sh lanechange      # one case
python tools/oracle/compare_xml.py <ref.xml> <cand.xml>   # exit 0 MATCH, 1 DIFF, 2 unreadable
```
