# Case: elechybrid_overhead (trolleybuses under overhead wires, device.elechybrid, traction substations)

Structure follows the official ElectricHybrid example (`docs/web/docs/Models/ElectricHybrid.md`) and
`tests/sumo/devices/elechybrid/elechybrid_2`, scaled down to a 3-node line with one junction.

Net: plain XML (`elecov.nod.xml`, `elecov.edg.xml`), 4 edges with 2 lanes each. The junction `nM` gives
internal lanes `:nM_0_*` (e2m to m2w) and `:nM_2_*` (w2m to m2e), so inner overhead segments are built.

Additional file `overhead.add.xml`:
- two `tractionSubstation` (`trSub1` 600 V, `trSub2` 700 V);
- eight `overheadWireSegment` (two voltage sources `ovh_w2m_0`, `ovh_e2m_0`, two with startPos/endPos);
- one `overheadWireClamp` (`clampA`, trSub1) and two `overheadWire` circuits, one with the clamp.

Vehicles (`routes.rou`): `trolleybus` vType with the device.elechybrid params (`maximumBatteryCapacity`,
`overheadWireChargingPower`, `maximumPower`, drag, mass, efficiencies). Three vehicles, two with
`actualBatteryCapacity` 23000 and one at 5000 Wh. Device forced with `--device.elechybrid.explicit`.

Reference (C++ 1.28.0, WSL, Eigen build, so the Eigen-based circuit solver is used):
```
bash net_ref.sh                              # ref_simple.net.xml
make_sim_refs.sh elechybrid_overhead         # sim.args = --end 120 + additional file + aggregated elechybrid output
```
Extra outputs (`extra_outputs.txt`): `ref.elechybrid.xml` (aggregated, precision 4),
`ref.overheadwires.xml` (overheadWireSegments-export), `ref.substations.xml` (substations-export).
