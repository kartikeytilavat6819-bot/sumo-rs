# tls_nema_loops

Four-arm traffic light `x` built with `--tls.default-type NEMA` (netconvert), with E1 loops (`det_W0`, `det_S0`,
150 m into each approach) from `tls.add.xml`. Cars on `W0→E0` and `S0→N0`, departing t=0–50 s.

- Sim: `--end 200 --additional-files tools/oracle/cases/tls_nema_loops/tls.add.xml`.
- Reference: `make_sim_refs.sh tls_nema_loops` (pristine C++ 1.28.0). Net: `net_ref.sh`.
