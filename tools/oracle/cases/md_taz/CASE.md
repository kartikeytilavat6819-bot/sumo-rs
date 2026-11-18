# md_taz

edgeData and laneData with `aggregate="taz"` (period 60), grouping the data of TAZ connector edges (declared via `<taz>` elements) into per-TAZ intervals. TAZ aggregation is otherwise untested in the oracle suite.

Sim: `--end 200 --additional-files tools/oracle/cases/md_taz/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_taz.xml`, `ref.ld_taz.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_taz; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
