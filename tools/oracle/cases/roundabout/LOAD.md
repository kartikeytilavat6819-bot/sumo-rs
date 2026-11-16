# Roundabout under load: ref_simple.net.xml

Net: `ref_simple.net.xml` (junction `439912279`, `type="priority"` with internal foes/cont lanes; the arms are `5197062#0` (NW), `-5197062#1` (E) and `37551440` (SW, bike/ped track, not usable by cars)).
Reference: C++ SUMO 1.28.0 (WSL). Ours: `C:\Users\karti\sumo-rs-target-main\debug\sumo.exe` (prebuilt, not rebuilt).

## Inputs

- `routes_load.rou`: 30 vehicles `l0`..`l29`, `depart = 3*i` (0–87 s), cycling 4 car routes:
  - rA `5197062#0 5197062#1`
  - rB `5197062#0 -5197062#0`
  - rC `-5197062#1 -5197062#0`
  - rD `-5197062#1 5197062#1`
- `routes_one.rou`: 1 vehicle `l0` on rA, depart 0.

Command (both sides, `--end 200`):

```
sumo -n ref_simple.net.xml -r <routes> --end 200 --tripinfo-output ... --fcd-output ... --statistic-output ... --no-step-log true
```

## Results

| Run | Arrivals ref / ours | Mean duration ref / ours | Total waitingTime ref / ours | Total travel time ref / ours |
|---|---|---|---|---|
| Load (30) | 30 / 30 | 43.9 s / 25.3 s | 369 s / 6 s | 1318 s / 760 s |
| Baseline (1) | 1 / 1 | identical | 0 / 0 | identical |

Last arrival: ref 143 s, ours 112 s.

Comparators:

| Check | Load (30) | Baseline (1) |
|---|---|---|
| `compare_sim --tripinfo` | DIFF `l4 arrivalSpeed: ref=13.92 cand=13.28` | MATCH |
| `compare_sim --fcd` | DIFF `t=22.00 l3 x: ref=197.82 cand=197.11` | MATCH |
| `compare_stat` | SUMMARY 5 differences in 5 groups (timeLoss 21.82 vs 3.23, totalTravelTime 1318 vs 760, waitingTime 12.30 vs 0.20) | SUMMARY 0 differences |

Teleports, emergency braking and collisions: 0 in both. Logs: the only warning is our `SUMO_HOME is not set properly` on stderr. The reference is silent.

## Circle occupancy (yield check)

The reference makes 16 of 30 vehicles wait (1–47 s each; `l9` 47 s, `l12` 39 s, `l13` 38 s). Ours make only 4 wait, for 1–2 s each. The wait pattern is consistent with our vehicles entering the junction while another vehicle occupies the conflicting lane, where the reference holds them back. I did not check occupancy frame-by-frame in FCD, so this is inferred from waiting time, not directly observed. The baseline matches exactly, so the gap appears only under load.

## Verdict

**Yielding is NOT honoured under load.** With a single vehicle we match the reference exactly. With 30 vehicles we let vehicles through the occupied junction that the reference makes wait, so our durations, waiting and total travel time are far too short. This is the specific gap-acceptance failure the test was designed to find. Fix belongs to a separate task; this run changed no code.

Not checked: full FCD occupancy timeline (only the first diff). The junction is `type="priority"`, not roundabout-typed, so "circle" here means the internal foe/cont lanes.
