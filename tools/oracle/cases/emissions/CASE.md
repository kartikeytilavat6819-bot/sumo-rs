Net: `ref_simple.net.xml` copied from `cases/junction` (netconvert v1.28.0, same input.osm).
Routes: `routes.rou` copied from `cases/junction` (5 vehicles, r0..r3).

Sim args: `sim.args` = `--end 200 --device.emissions.probability 1`, so every vehicle carries an
emissions device and tripinfo contains `<emissions>` children.

C++ refs (SUMO v1.28.0, WSL, `SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo`):
- `ref.tripinfo.xml`, `ref.fcd.xml`, `ref.stat.xml`: `--tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml --statistic-output ref.stat.xml --no-step-log true` + sim.args
- `ref.emission.xml`: same sim plus `--emission-output ref.emission.xml` (separate run; its tripinfo/fcd/stat went to /tmp)

Default emission class: `HBEFA4/default` (SUMOVTypeParameter.cpp EMPREFIX), resolves to `HBEFA4/PC_petrol_Euro-4` for DEFAULT_VEHTYPE.

Class variants: `sim_<v>.args` adds `--additional-files vt_<v>.add.xml`, which overrides DEFAULT_VEHTYPE's
`emissionClass` (C++ accepts the override). `<v>` is hbefa3 (HBEFA3/PC_G_EU4), phemlight (PHEMlight/PC_G_EU4),
phemlight5 (PHEMlight5/PC_EU4_G), energy (Energy/unknown, which C++ reports as `Energy/default`).
References: `ref_<v>.tripinfo.xml` (tripinfo, compared by run_all) and `ref_<v>.emission.xml` (emission output,
compared by hand like `ref.emission.xml`). Generated from `~/sumo-pristine/bin/sumo` with `--end 200 --device.emissions.probability 1`.
