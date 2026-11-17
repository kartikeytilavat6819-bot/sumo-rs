# Case: city_la_hov_ramps (net + simulation, freeway with HOV lane, on- and off-ramps)

Synthetic plain-XML freeway (not OSM) modelled on an LA freeway: eastbound mainline M0-M4 (400 m blocks, 3 lanes,
29.06 m/s), on-ramp R1 -> M1 and off-ramp M3 -> X3. The HOV lane is a separate parallel corridor H0-H4 (1 lane,
allow `hov taxi bus`), entered at M0 (`feed_in`) and left at M4 (`feed_out`). Mainline disallows `hov`.
Routes: 60 passenger cars, 10 trucks (depart [0, 400] s, random OD, seed 5), and 20 vClass `hov` vehicles that can only
use the HOV corridor (shortest path, no OD -> unreachable pairs are redrawn).

`sim.args` = `--end 400`. Reference: `netconvert <net.args> -o ref_simple.net.xml`, `make_sim_refs.sh city_la_hov_ramps` (NEW_CASE).
