# detectors: sim PASS, detector files PARTIAL

## run_all (sim stage)
- tripinfo, fcd and statistics PASS against C++ sumo with the same `additional.xml`.

## Detector files (manual, not in run_all)
Run in scratch directories with the same inputs, C++ and Rust, header lines removed:

- `det_il.xml` (inductionLoop `il_det`): MATCH, 19 data lines identical.
- `det_lad.xml` (laneAreaDetector `lad_det`): FAIL. First difference, interval 0-10 s:
  - C++: `sampledSeconds=4.69 nVehEntered=2 nVehLeft=1 nVehSeen=2 meanSpeed=14.51 meanOccupancy=6.25 meanVehicleNumber=0.60`
  - Rust: `sampledSeconds=0.00 nVehEntered=2 nVehLeft=0 nVehSeen=2 meanSpeed=-1.00 meanOccupancy=0.00 meanVehicleNumber=0.00`

Dispatch is now working for E2: `notify_enter` and `notify_move` run (entered and seen counts match). The gap is in the processing of the queued move notifications (`myMoveNotifications`), which is what produces sampled time, occupancy, leave count and speed. `detector_update` is now dispatched through the vtable, but the values stay zero, so the remaining bug is inside the E2 update or leave path. Not yet traced.
