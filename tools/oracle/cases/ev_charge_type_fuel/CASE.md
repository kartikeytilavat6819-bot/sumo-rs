# Case `ev_charge_type_fuel`: chargingStation `chargeType="fuel"` filtering

A `chargingStation cs_fuel` with `chargeType="fuel"` on `-103224#2_0`. With `--device.battery.track-fuel true`,
the HBEFA3 gasoline vehicle `gas` gets a FUEL battery device and charges at cs_fuel, while the `Energy/unknown`
vehicle `ev` has a NORMAL battery device whose charge type does not match, so it does not charge even though
it stops at the same station. Both vehicles have `has.battery.device=true`. Covers `chargeType` (not used by
any existing case) and track-fuel charging at a station (the existing track-fuel case has no station).
