# Case: city_swiss_tram_village (net + simulation, trams sharing car lanes, tram stops)

Synthetic plain-XML village (not OSM) modelled on a Swiss village street: main street V0-V4 (90 m blocks, one lane each way,
30 km/h), with a side street V2-Sn-Sw. NO tram track: trams (vClass `tram`) run in the same lanes as cars (default allow).
Routes: 40 cars (random OD, seed 6, depart [0, 500] s); 6 trams alternating forward (V0->V4) and back, each stopping 15 s
at two tram stops (`busStop` ts1..ts4 in `stops.add.xml`, loaded by `sim.args`).

`sim.args` = `--end 600 --additional-files tools/oracle/cases/city_swiss_tram_village/stops.add.xml`.
Reference: `netconvert <net.args> -o ref_simple.net.xml`, `make_sim_refs.sh city_swiss_tram_village` (NEW_CASE).
