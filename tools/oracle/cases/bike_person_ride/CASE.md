# bike_person_ride

Persons riding bicycles via `personTrip modes="bicycle"` on a three-node road
with sidewalks, plus one default-mode walking trip. Mirrors `person_modes_bike`
but uses a custom bicycle vType (maxSpeed 6.5 m/s). `personinfo` captures the
bicycle leg timings.

- Net: plain XML node/edge files with `--sidewalks.guess`, see `net.args`.
- Sim: `--end 150`. Refs: `make_sim_refs.sh bike_person_ride`.
