# cfmodels_new

Stress network (same `ref_simple.net.xml` as `cfmodels`, Times Square lights), `--end 200`.

- Base (`ref.*`): CACC platoon. A slow Krauss leader (`maxSpeed` 3) and four CACC vehicles p0..p3 on route A.
  Background vehicles b0..b2 use DEFAULT_VEHTYPE (Krauss).
- `KraussX` variant: DEFAULT_VEHTYPE is overridden to `carFollowModel="KraussX"` (`KraussX.add.xml`).

Refs: `bash tools/oracle/make_sim_refs.sh cfmodels_new` (C++ 1.28.0 from ~/sumo-pristine, WSL).
`NEW_CASE` marks a case with no committed refs yet, so every output and variant is written.
