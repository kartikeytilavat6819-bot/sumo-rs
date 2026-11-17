# cfmodels_rail

Rail network from `cases/rail` (`ref_simple.net.xml`: single track r1..r2 with a passing loop, rail signals r1/r2).
Deviation from the other cfmodels cases: Rail needs rail-capable edges, so there is no traffic light here; the rail signals hold trains.

- Base (`ref.*`): DEFAULT_RAILTYPE is a plain rail vehicle (`rail_base.add.xml`).
- `Rail` variant: DEFAULT_RAILTYPE uses `carFollowModel="Rail"` (`Rail.add.xml`).
- Traffic: `slowEast` (Krauss, maxSpeed 10) leads `followEast` (depart 8 s); `trainWest` runs the opposite way.

Refs: `bash tools/oracle/make_sim_refs.sh cfmodels_rail` (C++ 1.28.0 from ~/sumo-pristine, WSL).
