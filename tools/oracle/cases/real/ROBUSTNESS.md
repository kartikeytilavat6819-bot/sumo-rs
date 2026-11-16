# Robustness: real (Times Square) case

Ours: prebuilt `C:\Users\karti\sumo-rs-target-main\debug\sumo.exe` (not rebuilt).
Reference: C++ sumo 1.28.0 via WSL. Scratch outputs live outside the repo.

| Test | Ours | Reference | Verdict |
|------|------|-----------|---------|
| Long horizon `--end 600` (tripinfo + stat) | exit 0, no panic | exit 0 | PASS |
| Long horizon: teleport / emergency braking / collision warnings | none (stderr: only the SUMO_HOME warning) | none (stderr empty) | PASS |
| Long horizon: `compare_sim.py --tripinfo` | MATCH (6 vehicles) | | PASS |
| Long horizon: `compare_stat.py` | 0 differences in 0 groups (9 elements each) | | PASS |
| Determinism, default seed, 3 runs (tripinfo, fcd, stat excl. `<performance>`) | identical | | PASS |
| Determinism, `--seed 42`, 3 runs (tripinfo, fcd, stat excl. `<performance>`) | identical | | PASS |
| Empty route file (`routes_empty.rou`): exit code | 0 | 0 | PASS |
| Empty route file: tripinfo | valid, zero `<tripinfo>` | valid, zero `<tripinfo>` | PASS |
| Empty route file: `compare_sim.py --tripinfo` | MATCH | | PASS |

Notes:
- The determinism diffs that do exist are expected and not bugs: the `<!-- generated on -->` timestamp, the echoed output paths in the config header, and the wall-clock `<performance>` line in the stat file.
- The determinism check ran only our binary. The reference was not repeated.
