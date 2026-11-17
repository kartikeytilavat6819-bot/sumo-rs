# ft07_persons

Times Square net (`ref_simple.net.xml`, copied). One car and three persons (`personTrip` car/walk, `personTrip` walk, `walk`). `--seed 42`, `--end 200`.

- Base (`ref.*`): no extra options.
- `walkfactor`: `--persontrip.walkfactor 0.5`.
- `carwalk`: `--persontrip.transfer.car-walk parking`.

Refs: `bash tools/oracle/make_sim_refs.sh ft07_persons` (C++ 1.28.0 from ~/sumo-pristine, WSL). `NEW_CASE` marks a new case.
