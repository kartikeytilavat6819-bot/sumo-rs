# md_edgesfile

`edgesFile` selects the edges for edgeData and laneData from a text file (including an `edge:`-prefixed entry), instead of an inline `edges` list. Combined with `aggregate="true"` on the edgeData.

Sim: `--end 200 --additional-files tools/oracle/cases/md_edgesfile/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_ef.xml`, `ref.ld_ef.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_edgesfile; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
