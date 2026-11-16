Net: `ref_simple.net.xml` copied from `cases/junction` (netconvert v1.28.0, same input.osm).
Routes: `routes.rou` copied from `cases/junction` (5 vehicles, r0..r3).

Sim args: `sim.args` = `--end 200 --device.emissions.probability 1`, so every vehicle carries an
emissions device and tripinfo contains `<emissions>` children.

C++ refs (SUMO v1.28.0, WSL, `SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo`):
- `ref.tripinfo.xml`, `ref.fcd.xml`, `ref.stat.xml`: `--tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml --statistic-output ref.stat.xml --no-step-log true` + sim.args
- `ref.emission.xml`: same sim plus `--emission-output ref.emission.xml` (separate run; its tripinfo/fcd/stat went to /tmp)

Default emission class: `HBEFA4/default` (SUMOVTypeParameter.cpp EMPREFIX), resolves to `HBEFA4/PC_petrol_Euro-4` for DEFAULT_VEHTYPE.
