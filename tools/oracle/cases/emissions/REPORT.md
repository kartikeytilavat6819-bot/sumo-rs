# Emissions oracle case: report

Commit under test: `2b5d4806` (branch `worktree-agent-em01`). Gate kernel: `srg-em01-2b5d4806`,
`gate.txt` verified as `worktree-agent-em01 2b5d4806`.

## Results

| Check | Result |
|---|---|
| Build (`sumo`, `netconvert`) | pass |
| `cargo test -p sumo_core -p utils` | 64 + 115 passed, 0 failed |
| `<emissions>` in tripinfo (`compare_sim.py --tripinfo`, extended to child attrs) | MATCH |
| `--emission-output` vs `ref.emission.xml` (84 timestep rows) | 84/84 rows, 0 missing, 0 extra, 0 attribute diffs beyond 0.01 |

The emission-output comparison was done by hand in Python: rows keyed by (time, vehicle id),
compared attribute by attribute (`CO2 CO HC NOx PMx fuel electricity noise`, plus x/y/pos/speed/lane/route).
Tolerance is 0.011, the rounding of the 2-decimal C++ output.

## Defects found and fixed while getting here

1. `SumoXMLAttrMask` written-attributes default was empty. C++ `myWrittenAttributes(getDefaultMask())`
   sets all bits. Fixed in `ms_device_emissions.rs` (`6ad63d5d`). Without this every emission field was missing.
2. Emission values diverged from `t=1` (acceleration read as 0). C++ `MSBaseVehicle::getAcceleration()`
   is virtual and returns 0, and `MSVehicle` overrides it. The Rust `MSBaseVehicle` path does not dispatch,
   so speed/acceleration/slope are now read through `SUMOTrafficObject` (`2b5d4806`), which uses the vtable.

## Shortcuts that remain

- `MSDevice_Emissions` is built only through the `init_devices` path, which still skips unported devices
  (vehroutes, example, ...). Vehicles that would get those devices may not match C++ in output.
- `MSDetectorControl`: `gHaveEmissions |= typeid(*md) == MSMeanData_Emissions` is not set (no RTTI), so
  `--device.emissions` meso-mean-data is not reachable yet.
- `MSNet::write_output`: the netstate, battery, chargingstations, elechybrid, full, queue, amitran and vtk
  exports are still shortcuts. `full-output` only sets `gHaveEmissions`.
- Meso emission export (`use_meso_sim`) is not verified; the case is micro only.
- Only the junction case was checked. Other nets are not covered.

## Gate-tooling note

`remote_gate.ps1` names the dataset and kernel per branch. A rerun on a new commit on the same branch can
reuse the previous run's dataset and kernel and return stale results. Only the `WRONG SNAPSHOT` check caught it.
Naming the slot per commit avoided the problem in this run.

Other pre-existing failures in the same gate run: `city_munich` net, `zipper` net, and two depart variants
(`lane_1`, `pos_max`, CRLF in reference stderr). These match SCOREBOARD.md entries. They were not re-measured
against main in this same run.
