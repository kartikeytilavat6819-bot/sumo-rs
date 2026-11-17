# ft07_stepmeso

Times Square net (`ref_simple.net.xml`, copied), six vehicles on routes A and B. Step-length and meso options.

- Base (`ref.*`): `--step-length 0.1 --default.carfollowmodel IDM`, `--end 60`. vType `asl` has `actionStepLength="0.5"`.
- `action`: same plus `--default.action-step-length 0.5`.
- `meso`: `--mesosim` with `--meso-edgelength 50 --meso-tauff 1.5 --meso-jam-threshold 0.5 --meso-multi-queue true --meso-lane-queue true --lanechange.overtake-right true`, `--end 200`.

Refs: `bash tools/oracle/make_sim_refs.sh ft07_stepmeso` (C++ 1.28.0 from ~/sumo-pristine, WSL). `NEW_CASE` marks a new case.
