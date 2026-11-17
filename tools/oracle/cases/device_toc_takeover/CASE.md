# Case `device_toc_takeover`: takeover (ToC) device with a fired takeover and MRM

Net: `takeover.{nod,edg,con}.xml` (built by `net_ref.sh`, C++ netconvert in WSL). Lane 1 of `e1` is a turn-off
lane for the route `e0 e1 e2`. `toc_dyn` and `toc_mrm` depart on lane 1 of `e0`, so the device must request a
dynamic takeover (`dynamicToCThreshold 3`) before `n2`.

- `toc_dyn`: responseTime 6, the driver takes over (`DYNTOR`, `MRM`, `ToCdown` at t=5/7/7).
- `toc_mrm`: responseTime 12 > lead time, so the response is too slow (`DYNTOR` at t=9, `ToCdown` at t=10).

`--device.toc.probability 1` equips every vehicle; `extra_outputs.txt` compares `ref.toc.xml`.
Reference: C++ sumo 1.28.0 from `~/sumo-pristine` via `make_sim_refs.sh device_toc_takeover`.
