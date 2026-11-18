# Case `ev_probability_half`: global battery-device probability below 1

No vehicle carries `has.battery.device`; instead `--device.battery.probability 0.5` with a fixed `--seed 23`
equips a deterministic subset of the 8 vehicles. Existing cases use probability 1 with an explicit
`has.battery.device=true` parameter, so this covers the probabilistic equipment path and the `battery_*`
device IDs in tripinfo for only some vehicles.
