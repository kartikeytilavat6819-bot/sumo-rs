# out_stop_unfinished

`--stop-output.write-unfinished` with stops that outlast the run (duration 1000 s, sim ends at 150 s), so still-stopped vehicles are written at close.

Sim: `--end 150 --stop-output.write-unfinished true`.

Compared by run_all: tripinfo, fcd, stat and `ref.stop.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_stop_unfinished; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
