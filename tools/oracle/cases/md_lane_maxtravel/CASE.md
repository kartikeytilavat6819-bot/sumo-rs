# md_lane_maxtravel

laneData with `maxTraveltime="30"` and `minSamples="3"` (both previously tested only on edgeData), plus laneData with `withInternal="true"` (also previously edgeData-only).

Sim: `--end 200 --additional-files tools/oracle/cases/md_lane_maxtravel/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ld_mt.xml`, `ref.ld_int.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_lane_maxtravel; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
