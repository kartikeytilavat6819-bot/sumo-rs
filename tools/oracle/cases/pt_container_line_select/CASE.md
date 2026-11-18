# pt_container_line_select

Containers use `<transport>` with an explicit `lines` attribute on scheduled public lines.
Two carrier vehicles (lines **C1** and **C2**) serve `cs0`→`cs1`; containers request `C1`,
`C2`, or `ANY`. `--stop-output` captures loaded/unloaded containers and stop times.

Plain-XML net (two 200 m edges, `net.args`), `containerStop`s in the additional file.

Sim: `--end 300`, plus `--stop-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_container_line_select; rm NEW_CASE` (pristine C++ 1.28.0).
