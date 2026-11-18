# rr_persontrip

Person rerouting device: `--person-device.rerouting.probability 1 --person-device.rerouting.period 20
--person-device.rerouting.scope trip` on persons with `<personTrip modes="public">` and a bus line
(net and bus stops adapted from `out_person`). Exercises the person-device option plumbing and the
`trip` rerouting scope. `extra_outputs.txt` compares personroute and personsummary exactly.
Refs generated with pristine C++ 1.28.0 (WSL).
