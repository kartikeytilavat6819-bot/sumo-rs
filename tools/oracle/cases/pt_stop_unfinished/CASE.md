# pt_stop_unfinished

`--stop-output.write-unfinished` writes records for bus stops that have not ended by the
simulation end. Bus `hold` stops at `bs0` until time 200 (past `--end 100`), while a second
bus makes a short completed stop. Only `ref.stop.xml` captures the unfinished record.

Plain-XML net (two 200 m edges, `net.args`), `busStop`s in the additional file.

Sim: `--end 100 --stop-output.write-unfinished`, plus `--stop-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_stop_unfinished; rm NEW_CASE` (pristine C++ 1.28.0).
