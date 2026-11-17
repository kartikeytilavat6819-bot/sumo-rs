# Case: city_tokyo_scramble (net + simulation, signalised crossing, heavy pedestrian flow)

Synthetic plain-XML network (not OSM) modelled on the Shibuya scramble: a signalised 4-arm junction (`c`,
traffic_light, netconvert default program), 2 lanes each way on arms of 120 m. Sidewalk ring around the junction at corners +-10 m (four two-way sidewalk edges), plus pedestrian links from each
arm end to its two nearest corners. Vehicle edges disallow pedestrians and vice versa.
Found in testing: a pedestrian spoke into `c`, or a walk through an arm-end junction, gives "Disconnected walk" in
C++ 1.28 (no walking-area route across a road arm), so persons walk only between the four corners.
The scramble is therefore modelled only by the signalised junction and its one guessed crossing, not a true scramble.

`net.args` adds `--crossings.guess true --walkingareas true`. Routes: 40 cars (passenger, seed 3, depart [0, 300] s) and
120 persons (random origin/destination among the corners, depart [0, 300] s).
`sim.args` = `--end 400`.
Extra output: `--personinfo-output personinfo` (`extra_outputs.txt`, compared as `ref.personinfo.xml`).
Reference: `netconvert <net.args> -o ref_simple.net.xml`, then `make_sim_refs.sh city_tokyo_scramble` (NEW_CASE).
