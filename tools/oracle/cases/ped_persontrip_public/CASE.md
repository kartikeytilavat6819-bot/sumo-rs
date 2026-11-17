# ped_persontrip_public

Three 100 m edges E0 E1 E2 with sidewalks (`net.args`). Bus line L1 with stops `bs0` (E1) and `bs1` (E2) in `ped_pt.add.xml`
(stops on the driving lane `E1_1`/`E2_1`; lane 0 is the sidewalk).

- Persons with `personTrip modes="public"` (`pt0`..`pt4`) and plain walks (`pwalk`, `pwalk2`).
- `sim.args`: `--persontrip.walkfactor 0.75` → `ref.*`
- `sim_carwalk.args`: adds `--persontrip.transfer.car-walk parkingAreas,ptStops,allJunctions` → `ref_carwalk.*`
- `extra_outputs.txt`: `--personinfo-output` compared as `ref.personinfo.xml`.

Refs: `bash tools/oracle/make_sim_refs.sh ped_persontrip_public` (pristine C++ 1.28.0).
