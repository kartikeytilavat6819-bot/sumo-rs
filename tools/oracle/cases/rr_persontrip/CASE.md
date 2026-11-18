# rr_persontrip

Person rerouting device: `--person-device.rerouting.probability 1 --person-device.rerouting.period 20
--person-device.rerouting.scope trip` on persons with `<personTrip modes="public">` and a bus line
(net and bus stops adapted from `out_person`). Exercises the person-device option plumbing and the
`trip` rerouting scope. Fixed `--seed 5` and `sigma="0"` on the pedestrian type. `extra_outputs.txt`
compares personroute exactly (personsummary's per-step `duration` is timing-dependent, so it is not
compared). Refs generated with pristine C++ 1.28.0 (WSL).
