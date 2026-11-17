# Case: city_buenos_aires_oneway (net + simulation, SL2015 sublane)

Synthetic plain-XML grid (not OSM): 5 x 4 nodes, blocks 120 m (avenues, east-west) x 100 m (streets, north-south).
Layout modelled on the Buenos Aires Microcentro grid: east-west avenues are ONE-WAY (alternating east/west by row,
2 lanes), north-south streets are two-way (1 lane each way). Right-hand traffic.

`net.args` = plain node/edge files (`city_buenos_aires_oneway.nod.xml`, `.edg.xml`).
`sim.args` = `--end 600 --lateral-resolution 0.8` (SL2015 sublane model, as in the sublanes case).
Routes: 90 cars, depart uniform in [0, 400] s, random origin/destination (seed 1), shortest path on the one-way graph.

Reference: `netconvert <net.args> -o ref_simple.net.xml` and `make_sim_refs.sh city_buenos_aires_oneway` (NEW_CASE).
