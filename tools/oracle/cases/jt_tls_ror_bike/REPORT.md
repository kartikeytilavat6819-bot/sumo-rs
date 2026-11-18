# jt_tls_ror_bike — `traffic_light_right_on_red` with mixed vClasses

4-arm node type `traffic_light_right_on_red`, 20 vehicles cycling through
`passenger`, `bicycle`, `motorcycle` and `truck` vTypes. The committed
`jt_traffic_light_right_on_red` uses default cars only; this tests right-on-red
behaviour for non-car classes (notably bicycles).

Net: C++ netconvert 1.28.0; sim refs: C++ sumo 1.28.0, `--end 200`.
