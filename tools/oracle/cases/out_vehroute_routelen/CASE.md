# out_vehroute_routelen

`--vehroute-output` with `--vehroute-output.route-length` and `--vehroute-output.sorted` on the junction net (no rerouting).

Sim: `--end 150 --vehroute-output.route-length true --vehroute-output.sorted true`.

Compared by run_all: tripinfo, fcd, stat and `ref.vehroute.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_vehroute_routelen; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
