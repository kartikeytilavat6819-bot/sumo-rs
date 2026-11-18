# md_period300

edgeData and laneData with `period="300"` covering the whole 300 s run in a single interval (period equal to the simulation horizon).

Sim: `--end 300 --additional-files tools/oracle/cases/md_period300/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed300.xml`, `ref.ld300.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_period300; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
