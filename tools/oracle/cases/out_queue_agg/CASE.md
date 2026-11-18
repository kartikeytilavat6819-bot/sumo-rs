# out_queue_agg

`--queue-output` with `--queue-output.aggregation 50` and `--queue-output.skip-empty true` on the junction net (period 5).

Sim: `--end 150 --queue-output.period 5 --queue-output.aggregation 50 --queue-output.skip-empty true`.

Compared by run_all: tripinfo, fcd, stat and `ref.queue.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_queue_agg; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
