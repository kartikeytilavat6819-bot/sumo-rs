# Case `ev_mmpevem`: ev case with MMPEVEM emission class, precision 6

Same net, charging station and trips as `cases/ev`; the EV vType is `MMPEVEM/unknown` (`routes.rou`, sed of the ev routes).
`sim.args` adds `--precision 6`. Reference: C++ sumo 1.28.0 from `~/sumo-pristine`, flags as in `sim.args`
plus `--battery-output`, `--emission-output`, `--chargingstations-output` (`ref.*`).
Battery, emission and charging-station outputs are compared with `cases/ev/compare_precision.py ref ours`.
