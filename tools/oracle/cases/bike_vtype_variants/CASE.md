# bike_vtype_variants

Bicycle vTypes differing only in `maxSpeed` (3/5/7 m/s), `accel` (0.5/1/2/3)
and `decel` (1/2/3.5/4.5), on a single 400 m edge whose speed (20 m/s) is above
all bike maxima. FCD speed traces and tripinfo arrival times isolate the
car-following parameter effects for the bicycle class.

- Net: plain XML (1-lane edge `E0`), see `net.args`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_vtype_variants`.
