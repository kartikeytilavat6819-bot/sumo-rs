# md_persons

edgeData and laneData with `detectPersons="walk"` plus a multi-mode `detectPersons="walk bicycle car public"` (period 60), with walking persons sharing edges with car flows. Person-mode detection in mean data is otherwise untested.

Sim: `--end 200 --additional-files tools/oracle/cases/md_persons/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_ped.xml`, `ref.ed_pedall.xml`, `ref.ld_ped.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_persons; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
