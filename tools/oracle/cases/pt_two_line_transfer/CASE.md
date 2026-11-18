# pt_two_line_transfer

Pedestrians use `personTrip modes="public"` from E0 to E2 where no single line covers the
whole trip: line **L1** runs E0→E1 and line **L2** runs E1→E2, sharing the transfer stop
`bsM` on E1. The router must board L1 at `bsA`, transfer at `bsM`, then board L2 to `bsZ`.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--stop-output` and `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_two_line_transfer; rm NEW_CASE` (pristine C++ 1.28.0).
