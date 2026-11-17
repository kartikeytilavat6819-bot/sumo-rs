# jmred

Red-light junction-model parameters on a single signal `n1` (static 15 s green / 4 s yellow / 16 s red), chain `E0 -> E1`.

- `vDar`: `jmDriveAfterRedTime=4` (may drive through red after waiting).
- `vKc`: `jmIgnoreKeepClearTime=2`, `jmDriveAfterRedTime=0`.
- `vStd`: defaults. Flows every 7 s, offset 0/2/4 s, so some vehicles arrive on red.

Sim: `--end 120`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
