# out_edgedata_internal

edgeData with `withInternal="true"` alone (all edges, period 50) on the junction net, 200 s.

Sim: `--end 200 --additional-files tools/oracle/cases/out_edgedata_internal/edgedata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_in.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_edgedata_internal; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
