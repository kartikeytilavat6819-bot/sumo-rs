# md_typealias

Mean-data type aliases: `type="performance"` (alias for the traffic type) and the deprecated `type="hbefa"` (emissions type, C++ prints a deprecation warning). Both are otherwise untested.

Sim: `--end 200 --additional-files tools/oracle/cases/md_typealias/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_perf.xml`, `ref.ed_hbefa.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_typealias; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
