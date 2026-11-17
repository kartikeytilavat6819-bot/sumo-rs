# ped_stop_lat

Two 100 m edges E0 E1 with sidewalks (`net.args`). Pedestrians (`ps0`..`ps4`) use person `<stop>` waiting stages
(`lane="E0_0"`, `duration` and `until`), `departPos`, `departPosLat`, `arrivalPos`, `arrivalPosLat`, walk `speed`.
Cars on E0 E1 feed the default FCD output.

- `sim.args`: `--end 200` → `ref.*` (tripinfo, fcd, stat, personinfo)
- `sim_legacy.args`: `--pedestrian.striping.legacy-departposlat` → `ref_legacy.*`

Refs: `bash tools/oracle/make_sim_refs.sh ped_stop_lat` (pristine C++ 1.28.0).
