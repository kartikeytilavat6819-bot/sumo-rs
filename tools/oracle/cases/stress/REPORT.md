# Stress case: 120-vehicle oversaturation on Times Square

Network: `ref_simple.net.xml` (copy of `../real/ref_simple.net.xml`).
Routes: `routes.rou`, 120 vehicles `s0`..`s119`, `depart = i` (0–119 s), `departSpeed="max"`.
Four overlapping routes (i % 4) all sharing the `1117867028 -> 1117867029#*` corridor, so queues form on the same approach.

## Commands

```
# C++ reference (WSL, sumo 1.28.0)
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou --end 200 \
  --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml --statistic-output ref.stat.xml --no-step-log true

# Ours (prebuilt C:\Users\karti\sumo-rs-target-main\debug\sumo.exe)
sumo.exe -n ref_simple.net.xml -r routes.rou --end 200 \
  --tripinfo-output ts_tripinfo.xml --fcd-output ts_fcd.xml --statistic-output ts_stat.xml --no-step-log true

python ../../compare_sim.py --tripinfo ref.tripinfo.xml ts_tripinfo.xml
python ../../compare_sim.py --fcd ref.fcd.xml ts_fcd.xml
python ../../compare_stat.py ref.stat.xml ts_stat.xml
```

## Outcome

- Ours: exit 0, no panic. Only stderr line: `Warning: Environment variable SUMO_HOME is not set properly, disabling XML validation`.
- C++: exit 0, empty stderr/stdout.
- Wall clock: C++ 0.54 s (includes `wsl` launch overhead); ours 3.54 s. Not a like-for-like benchmark.

## Counts (end = 200 s)

| | Reference | Ours |
|---|---|---|
| Inserted | 101 | 108 |
| Arrived (tripinfo entries) | 93 | 107 |
| Running at end | 8 | 1 |
| Waiting (never inserted) | 19 | 12 |

Neither side has duplicate vehicle IDs. No `teleport` or `emergencyStop` entries in either tripinfo.

## First divergences

Comparators stop at the first diff.

- `compare_sim --tripinfo`: `DIFF: vehicle s24 field depart: ref='43.00' cand='48.00'`
- `compare_sim --fcd`: `DIFF: t=43.00 vehicle s22 field x: ref='136.87' cand='134.48'`
  - Both sides have `s22` on `682360554#1_3` at 43 s region; the position differs by about 2.4 m.
  - Insertion-delayed departures are the first sign of divergence: the same queue at the start of the corridor releases vehicles at different times.

## compare_stat SUMMARY

`SUMMARY: 12 differences in 12 groups (ref 9 elements, cand 9 elements)`

Differing groups: `vehicles inserted` (101 vs 108), `running` (8 vs 1), `waiting` (19 vs 12), `vehicleTripStatistics count` (93 vs 107), `departDelay` (16.23 vs 36.17), `duration` (20.28 vs 13.85), `routeLength` (51.14 vs 73.19), `speed` (5.77 vs 6.75), `timeLoss` (15.29 vs 6.79), `totalDepartDelay` (1509 vs 3870), `totalTravelTime` (1886 vs 1482), `waitingTime` (9.64 vs 0.01).

## Warning/teleport kinds

- Ours: one `Warning` (SUMO_HOME not set). Reference: no warnings. Different kind, not just count.
- No teleport or emergency braking messages in either run.

## Verdict

**Does not match reference under oversaturation: no panic, completes, but first diverges at s24 (depart 43 s ref vs 48 s ours, on the 1117867028 corridor) and delivers 107 vs 93 arrivals. Not a threshold: no sub-120 load was run.**

## Caveats

- Ours is the prebuilt `main` binary, not necessarily the exact `worktree-agent-rb01` HEAD.
- Timing includes `wsl` launch overhead on the reference side.
- Comparators stop at the first diff, so later diffs are not enumerated.
