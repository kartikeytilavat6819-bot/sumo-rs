# routeprobe — FAIL (ours panics)

C++ reference (`make_sim_refs.sh routeprobe`, pristine 1.28.0), ours from the VM gate build of this
branch (`run_all.sh`):

```
routeprobe/ref  sim-PANIC  panicked at crates/sumo_core/src/microsim/output/ms_detector_file_output.rs:28:5:
                           pure virtual MSDetectorFileOutput::writeXMLDetectorProlog, subclasses implement it
```

First differing quantity: `v0` is missing from our tripinfo (we die at t=0, before any movement).
C++ numbers (`ref.tripinfo.xml`): `v0`/`v1` arrive on `E3_0` with `rerouteNo=0`, `v2`…`v7` on
`E4_0` with `rerouteNo=1` — the `routeProbReroute` at t=40 is exercised.

Cause to look at: `MSRouteProbe` must override `writeXMLDetectorProlog` (C++
`MSRouteProbe::writeXMLDetectorProlog` writes the `routes_file.xsd` header); the Rust subclass does
not, so the socket-detector file prolog hits the base-class pure-virtual stub. The probe output file
it should write is committed as `ref.probe.xml`.
