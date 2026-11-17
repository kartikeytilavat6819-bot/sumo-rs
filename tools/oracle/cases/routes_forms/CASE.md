# routes_forms

Route-file forms that used to panic: a `<flow>` with an inline `<route>` child, a `<trip from to>`
without a route, and a `<vehicle>` with an inline route. Net: `junction/ref_simple.net.xml`.

Reference: `SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou --end 200 --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml --statistic-output ref.stat.xml --no-step-log true` (WSL). `sim.args` = `--end 200`.

Not covered (on purpose): `<stop>` inside route files. Stop children parse, but a stop on the edge a
vehicle is leaving panics in `ms_vehicle.rs` leaveLane (stop execution is not ported). Add them back
once that is ported. A trip whose `from`/`to` are not adjacent also needs the router, which the sim does not have.
