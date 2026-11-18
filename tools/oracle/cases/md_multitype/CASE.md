# md_multitype

`type="emissions harmonoise"` on edgeData: a space-separated list of mean-data types, where the first device writes the main output and subsequent types are chained as sub-data into the same file. Untested by existing cases (which use a single type). LaneData with multiple types is omitted because SUMO 1.28.0 itself emits a duplicate `traveltime` attribute there.

Sim: `--end 200 --additional-files tools/oracle/cases/md_multitype/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_multi.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_multitype; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
