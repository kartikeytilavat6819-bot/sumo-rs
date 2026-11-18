# Case `emis_hbefa4_diesel`: HBEFA4 diesel emission class

Net and routes copied from `cases/junction` (5 vehicles). `vt.add.xml` sets DEFAULT_VEHTYPE to `HBEFA4/PC_diesel_Euro-4`.
`sim.args`: `--end 200 --device.emissions.probability 1`; `extra_outputs.txt`: `--emission-output emission`.
Reference: C++ sumo 1.28.0 (WSL, `~/sumo-pristine`) via `make_sim_refs.sh` (NEW_CASE).
