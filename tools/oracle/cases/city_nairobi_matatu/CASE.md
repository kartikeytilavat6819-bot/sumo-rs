# Case: city_nairobi_matatu (net + simulation, roundabout, matatu buses at a bus stop)

Synthetic plain-XML network (not OSM), left-hand traffic (`--lefthand`, Kenya).
A 4-node roundabout (ring radius 25 m, one-way clockwise, guessed by netconvert) with four 200 m two-way arms N/E/S/W.
Routes: 40 cars (random OD, seed 2, depart [0, 300] s) and 8 matatus (vClass `bus`, type `matatu`) from W/S/N to E,
each stopping 20 s at busStop `bs_e` on lane `E_out_0` (`stops.add.xml`, loaded by `sim.args`).

`sim.args` = `--end 600 --additional-files tools/oracle/cases/city_nairobi_matatu/stops.add.xml`.
Reference: `netconvert --lefthand <net.args> -o ref_simple.net.xml`, then `make_sim_refs.sh city_nairobi_matatu` (NEW_CASE).
