# vclass_speed_restrict

Edge type `highway` with `<restriction>` speeds for six vClasses (passenger, bus, truck, hov, taxi,
emergency), each vehicle having `maxSpeed` above its restriction. FCD `speed` must follow the per-vClass
type restriction, not the vehicle `maxSpeed`. Sim: `--end 130`.