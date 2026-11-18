# md_window

edgeData and laneData with explicit `begin`/`end` windows (period 50), testing intervals that start mid-run and stop before the simulation end.

Sim: `--end 300 --additional-files tools/oracle/cases/md_window/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_win.xml`, `ref.ld_win.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_window; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
