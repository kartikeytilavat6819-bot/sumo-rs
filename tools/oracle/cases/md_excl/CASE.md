# md_excl

laneData `excludeEmpty` in all three forms: default (omitted), `true`, and `false` (period 50). Empty lanes (no traffic in an interval) are dropped only with `true`.

Sim: `--end 200 --additional-files tools/oracle/cases/md_excl/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ld_ex_def.xml`, `ref.ld_ex_true.xml`, `ref.ld_ex_false.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_excl; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
