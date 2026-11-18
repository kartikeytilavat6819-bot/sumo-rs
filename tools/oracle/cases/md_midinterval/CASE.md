# md_midinterval

Vehicles that depart mid-interval (a flow starting at t=27 and a single vehicle at t=63) with `trackVehicles` edgeData/laneData at period 50; tests partial-interval vehicle entry and the `entered`/`sampledSeconds` counters.

Sim: `--end 150 --additional-files tools/oracle/cases/md_midinterval/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_mid.xml`, `ref.ld_mid.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_midinterval; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
