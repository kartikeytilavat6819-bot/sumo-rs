# cfg_vehroute_dua

`--vehroute-output` with `--vehroute-output.dua true` (DUA-style route output), `--vehroute-output.sorted true` and `--vehroute-output.write-unfinished true`, stopped at 60 s so vehicles are still driving. Existing cases cover sorted and write-unfinished separately; this one adds dua-mode.

Sim: `--end 60 --vehroute-output.dua true --vehroute-output.sorted true --vehroute-output.write-unfinished true`.

Compared by run_all: tripinfo, fcd, stat and `ref.vehroute.xml` (compare_xml.py, exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_vehroute_dua; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
