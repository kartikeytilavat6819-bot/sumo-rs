# calibrator_multi

Calibrator `cal0` on E1 at 50 m with three inline `<flow>` intervals (720 veh/h at 10 m/s for 0-60 s, 360 veh/h at 8 m/s for 60-120 s, 1080 veh/h at 12 m/s for 120-200 s), all on route `r0` (E0 E1). Exercises the multi-interval flow parse and interval switching in MSCalibrator.
Net and routes are copied from `cases/calibrator`. No type attribute on the flows (the additional file loads before `routes.rou`).
Sim: `--end 200 --additional-files tools/oracle/cases/calibrator_multi/calibrator_multi.add.xml`. Reference: tripinfo only, via `make_sim_refs.sh calibrator_multi`.
