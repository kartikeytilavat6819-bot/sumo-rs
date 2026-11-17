# out_tls

TLS timedEvent outputs (SaveTLSStates with saveDetectors, SaveTLSSwitchTimes, SaveTLSSwitchStates, SaveTLSProgram) on the `tls_waut` net, routes and WAUT (static progA -> actuated progB -> progA). The fcd is identical to `tls_waut`, so it is not committed again.

Sim: `--end 320 --additional-files tools/oracle/cases/out_tls/waut.add.xml,tools/oracle/cases/out_tls/tlsout.add.xml`.

Compared by run_all: tripinfo, stat and `ref.<suffix>.xml` for tls_states, tls_switch_times, tls_switch_states, tls_prog (compare_xml.py, exact).

`extra_outputs.txt` = command-line outputs; `add_outputs.txt` = files written by the additional file, resolved next to it (run_all/make_sim_refs move/compare them).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_tls; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
