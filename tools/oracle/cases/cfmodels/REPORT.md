# cfmodels: one variant per car-following / lane-change model

Base: `stress` network and routes (120 vehicles, Times Square, queues at the lights), `--end 200`.
Each variant overrides `DEFAULT_VEHTYPE` through `<model>.add.xml` and runs with `sim_<v>.args`.

- Reference: C++ SUMO 1.28.0, `bash tools/oracle/make_sim_refs.sh cfmodels` (WSL).
- Ours: `sumo-rs-target-x07\debug\sumo.exe`, run, not built.
- Compared: tripinfo (`compare_sim.py`), FCD (`compare_sim.py --fcd`), statistics (`compare_stat.py`). Verdict PASS = all three match.

## Results

| variant | verdict | first diff / note |
|---|---|---|
| Krauss | PASS | |
| KraussOrig1 | PASS | |
| KraussPS | FAIL | t=113 FCD extra timestep; `s84` extra in ours; inserted 70 (C++) vs 101 (ours) |
| SmartSK | PASS | |
| Daniel1 | PASS | |
| IDM | PASS | |
| IDM_delta2 (`delta="2"`) | PASS | non-default IDM delta |
| IDMM | PASS | |
| EIDM | PASS | |
| ACC | PASS | |
| CACC | FAIL | t=195 FCD extra timestep; `s70` extra in ours; running 11 (C++) vs 9 (ours) |
| Wiedemann | PASS | |
| W99 | PASS | |
| W99_cc1 (`cc1="1.0"`) | FAIL | t=39 FCD extra timestep; `s20` extra in ours; inserted 27 (C++) vs 96 (ours); emergencyBraking 0 vs 1 |
| PWagner2009 | PASS | |
| BKerner | PASS | |
| CC (`lanesCount="2"`) | PASS | CC needs `lanesCount` on the vType; without it C++ exits 1 |
| Rail | SKIP | road-only net; Rail needs rail vehicles and a rail network, none in the stress base |
| lc_DK2008 (`laneChangeModel`) | PASS | Krauss car-following |
| lc_LC2013_CC (`laneChangeModel`) | PASS | Krauss car-following |

Sanity: every model gives a distinct tripinfo on both sides, so the `DEFAULT_VEHTYPE` override takes effect
(a no-op override would have made all variants identical to `ref.tripinfo.xml`).

## Findings

- **W99 with `cc1`**: largest gap. Default W99 matches exactly, but `cc1=1.0` diverges at t=39 and the
  insertion count differs by 69 vehicles.
- **KraussPS** and **CACC**: both diverge later (t=113, t=195) on a single extra vehicle in ours.
- Divergences are not from ignored attributes: our outputs differ per model (md5 check).

## Not done

- Rail: needs a rail-only network (not built).
- `vtypes` has no CASE.md on disk; this report follows the scoreboard instead.
- No `run_all.sh` run; the variants were run directly with the comparators, same flags as `run_all.sh`.
