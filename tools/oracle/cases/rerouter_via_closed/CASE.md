# Rerouter: via reroute around a closed edge

Tests the via branch of `MSTriggeredRerouter::triggerRouting`: the router that computes the remaining part
(via edge -> destination) must carry the closed-edge prohibitions (`rerouteDef->getClosed()`).

- Net: `ref_simple.net.xml` = `netgenerate --grid --grid.number=4 --grid.length=100 --no-turnarounds` (pristine 1.28.0).
- Routes: `routes.rou`, route `A0B0 B0C0 C0C1 C1C2 C2D2 D2D3`, four cars.
- Rerouter on `A0B0`: `closingReroute C2D2` + `viaProbReroute B1B2`. The shortest way from the via edge to `D2D3`
  uses `C2D2`. C++ result: vehicles are rerouted to the via edge and end there (`A0B0 B0B1 B1B2`): the remaining
  part (B1B2 -> D2D3) does not start at the current edge, so replaceRouteEdges rejects it.
- Flags: `sim.args`; extra output: `--vehroute-output` (shows the replaced routes).
