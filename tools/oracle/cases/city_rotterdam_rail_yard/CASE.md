# Case: city_rotterdam_rail_yard (net + simulation, freight trains, rail signals)

Synthetic plain-XML rail network (not OSM) modelled on a port rail yard: a double-track main line W-J1-J2-E
(400 m each side) with rail-signal junctions J1, J2 (node type `rail_signal`), and a one-way yard loop on each side
(Y1/Y2 at 60 m offset; loop A J1-Y1-Y2-J2, loop B J2-Y2-Y1-J1). All edges `allow="rail"`, 20 m/s.
Routes: 12 freight trains (vClass rail, length 150 m), cycling through: direct E1-E2-E3, yard loop A, direct W, yard loop B;
departures every 50 s plus jitter (seed 4).

`sim.args` = `--end 900`. Reference: `netconvert <net.args> -o ref_simple.net.xml`, `make_sim_refs.sh city_rotterdam_rail_yard` (NEW_CASE).
