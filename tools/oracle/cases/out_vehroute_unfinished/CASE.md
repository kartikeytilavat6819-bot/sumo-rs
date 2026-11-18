# out_vehroute_unfinished

`--vehroute-output` with `--vehroute-output.write-unfinished` and `--vehroute-output.exit-times` on the junction net, stopped at 60 s so vehicles are still driving.

Sim: `--end 60 --vehroute-output.write-unfinished true --vehroute-output.exit-times true`.

Compared by run_all: tripinfo, fcd, stat and `ref.vehroute.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_vehroute_unfinished; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
