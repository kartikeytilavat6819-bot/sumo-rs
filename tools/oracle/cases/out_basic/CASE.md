# out_basic

Summary (period 10), queue (period 5), lanechange output (started/ended/xy) and tripinfo write-unfinished/write-undeparted with max-depart-delay, on the junction net.

Sim: `--end 150 --summary-output.period 10 --queue-output.period 5 --lanechange-output.started true --lanechange-output.ended true --lanechange-output.xy true --tripinfo-output.write-unfinished true --tripinfo-output.write-undeparted true --max-depart-delay 30`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for summary, queue, lanechange (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_basic; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
