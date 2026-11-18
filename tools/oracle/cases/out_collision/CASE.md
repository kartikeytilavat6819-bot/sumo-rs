# out_collision

Collision output with junction collisions (`--collision.check-junctions true`, drivers with jmIgnoreFoeProb/jmIgnoreJunctionFoeProb 1), action warn with stoptime 3 (`ref`) and action remove (`ref_remove`).

Sim: `--end 200 --collision.action warn --collision.check-junctions true --collision.stoptime 3`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for collision (compare_xml.py, exact).
Variant `ref_remove`: `--end 200 --collision.action remove --collision.check-junctions true`.
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_collision; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
