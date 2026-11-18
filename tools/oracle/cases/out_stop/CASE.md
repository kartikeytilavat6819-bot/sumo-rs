# out_stop

`--stop-output` on the junction net with four vehicles given explicit `<stop>` elements (lane stops; one vehicle without a stop as control).

Sim: `--end 200`.

Compared by run_all: tripinfo, fcd, stat and `ref.stop.xml` (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_stop; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
