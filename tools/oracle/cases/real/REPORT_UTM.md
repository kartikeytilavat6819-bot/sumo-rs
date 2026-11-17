# UTM net stage (no `--simple-projection`)

Runner: `bash tools/oracle/run_utm.sh [--refs]` (Git Bash). `--refs` regenerates `ref_utm.net.xml` from the untouched SUMO in WSL (`~/sumo-pristine`). The rest runs our `netconvert.exe` from `sumo-rs-target-x07` and compares with `compare_net.py`. Also checks that `<location projParameter>` matches exactly.

Binary: `sumo-rs-target-x07/debug/netconvert.exe`, built from main `9a81b30d` (contains the UTM port).

| case | net | projParameter (ref = ours) | first diffs |
|---|---|---|---|
| real | PASS | `+proj=utm +zone=18 +ellps=WGS84 +datum=WGS84 +units=m +no_defs` | none |
| signal | PASS | `+proj=utm +zone=28 ...` | none |
| city_plano | PASS | `+proj=utm +zone=14 ...` | none |
| city_munich | FAIL | `+proj=utm +zone=32 ...` (match) | 12 differences, all junction type or connection state (`zipper`/`priority`, `Z`/`M`/`m`); no coordinate diffs |

city_munich's 12 differences are the same count as its known `--simple-projection` FAIL (SCOREBOARD: "12 junction-classification diffs"). Projection and coordinates agree, so this is the existing junction-classification gap, not the UTM port.

Summary: UTM net stage PASS=3 FAIL=1. Projection parameter matches on all four.
