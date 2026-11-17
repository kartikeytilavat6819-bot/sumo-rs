# tls_delay_loops

Same four-arm crossing as `tls_nema_loops`, but the traffic light is `--tls.default-type delay_based`
with E1 loops (`det_W0`, `det_S0`, 150 m into each approach) from `tls.add.xml`.

- Sim: `--end 200 --additional-files tools/oracle/cases/tls_delay_loops/tls.add.xml`.
- Reference: `make_sim_refs.sh tls_delay_loops` (pristine C++ 1.28.0). Net: `net_ref.sh`.
