# bike_guess_lane

Bike lanes guessed by netconvert from a single two-way residential OSM way via
`--bikelanes.guess`: each direction gets a bicycle-only lane (lane 0) beside a
car lane. Bicycles (`bike0..bike2`, `bikeR0`) and passenger cars share edge `10`
and `-10`; the FCD `lane` attribute shows whether each class picks its lane.

- Net: `netconvert --osm-files input.osm --simple-projection --bikelanes.guess`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_guess_lane`.
