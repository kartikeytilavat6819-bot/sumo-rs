# harmonoise

Net: `ref_simple.net.xml` and `routes.rou` copied from `cases/emissions` (same junction net, 5 vehicles).

Sim args: `sim.args` = `--end 200`. `sim_edgedata.args` adds `edgedata.add.xml`, an edgeData with
`type="harmonoise"` (`freq="60"`), so the MSMeanData_Harmonoise constructor is exercised
(`maxTravelTime` and `minSamples` order, see NLDetectorBuilder).

C++ refs (SUMO v1.28.0, `~/sumo-pristine/bin/sumo`, WSL):
- `ref.tripinfo.xml`: `--end 200`
- `ref_edgedata.tripinfo.xml` + `ref_edgedata.edge.xml`: sim_edgedata.args; the edge output is written next to
  the additional file as `out_edgedata.xml` and renamed to `ref_edgedata.edge.xml`. Noise values are nonzero
  (e.g. `-103224#2 noise="66.90"`).
