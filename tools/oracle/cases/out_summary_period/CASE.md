# out_summary_period

`--summary-output` with `--summary-output.period 25` on the junction net, 200 s.

Sim: `--end 200 --summary-output.period 25`.

Compared by run_all: tripinfo, fcd, stat and `ref.summary.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_summary_period; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
