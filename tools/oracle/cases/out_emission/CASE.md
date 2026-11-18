# out_emission

`--emission-output` (per-vehicle emission records) with `--emission-output.precision 3` on the junction net.

Sim: `--end 150 --emission-output.precision 3`.

Compared by run_all: tripinfo, fcd, stat and `ref.emission.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_emission; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
