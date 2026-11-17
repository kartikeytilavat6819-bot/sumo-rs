Command (net): `SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/netconvert --osm-files input.osm --simple-projection --tls.default-type NEMA -o ref_simple.net.xml`
Command (sim): `SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml --statistic-output ref.stat.xml --no-step-log true --end 200`
Inputs: `input.osm`, `routes.rou` copied from `cases/signal`. Flags: `net.args` = `--tls.default-type NEMA`, `sim.args` = `--end 200`.
