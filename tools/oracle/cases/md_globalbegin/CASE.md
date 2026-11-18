# md_globalbegin

Simulation with global `--begin 50 --end 200`; the mean-data `begin`/`end` are omitted so they default to the global options. Tests the global-begin interaction with edgeData/laneData interval boundaries.

Sim: `--begin 50 --end 200 --additional-files tools/oracle/cases/md_globalbegin/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_gb.xml`, `ref.ld_gb.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_globalbegin; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
