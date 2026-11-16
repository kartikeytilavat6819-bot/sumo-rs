# Left-hand traffic case

Covers `lefthand` (`NLHandler` reads the flag from `<location>`; `MSGlobals::gLefthand` drives the lateral
sign in `MSVehicle` and the LC2013/SL2015 lane-change direction).

- Net: `input.osm` (same as lanechange), `ref_simple.net.xml` built by C++ netconvert **with `--lefthand`**
  (`make_net_ref`-style: `netconvert --osm-files ... --simple-projection --lefthand`).
- Routes: `routes.rou` — `b0` slow on lane 0, followers on both lanes so a lane change is needed.
- Flags: `sim.args` = `--end 120`.

**Known harness limitation:** `run_all.sh`'s net stage runs netconvert without `--lefthand`, so the net stage
always reports a difference (mirrored lane and junction shapes) for this case. The net stage is expected to
FAIL; the sim stage is the real test. Fixing this needs a per-case netconvert flag in `run_all.sh`, which this
task did not allow.
