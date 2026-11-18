# md_noperiod

edgeData and laneData without a `period` attribute; C++ sets the sampling frequency to `end - begin` (here 20..170, one interval). Tests the implicit-period path.

Sim: `--end 200 --additional-files tools/oracle/cases/md_noperiod/meandata.add.xml`.

Compared by run_all: tripinfo, fcd, stat and `ref.ed_np.xml`, `ref.ld_np.xml` (compare_xml.py, exact).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh md_noperiod; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
