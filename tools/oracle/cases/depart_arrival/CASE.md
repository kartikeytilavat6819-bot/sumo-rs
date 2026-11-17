# depart_arrival

Seven cars on a two-edge route `P0 P1` with each `departSpeed` variant (`desired`, `speedLimit`, `avg`, `max`, `random`)
and arrival variants (`arrivalLane` current/0/2/1, `arrivalPos` max/100/random/center, `arrivalSpeed` current/5/10).

- Sim: `--end 200 --seed 7` (`random` departSpeed/arrivalPos draw from the seed). Reference: `make_sim_refs.sh depart_arrival`.
- Net: `net_ref.sh`.
