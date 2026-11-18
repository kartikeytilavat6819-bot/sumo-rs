# md_amitran

edgeData and laneData with `type="amitran"` (period 60), the Amitran aggregate-output format, which is not covered by the existing command-line `--amitran-output` case.

Sim: `--end 200 --additional-files tools/oracle/cases/md_amitran/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_am.xml`, `ref.ld_am.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_amitran; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
