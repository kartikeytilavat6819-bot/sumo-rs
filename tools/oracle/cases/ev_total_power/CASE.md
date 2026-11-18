# Case `ev_total_power`: chargingStation `totalPower` limit with concurrent in-transit charging

`chargingStation cs_limited` with `power="100000"` but `totalPower="5000"`, `chargeInTransit="true"`,
spanning 20-100 m of `-103224#2_0`. Three EVs depart 1 s apart so several are inside the station area at
the same timestep, forcing MSChargingStation::checkTotalPower to rebalance the delivered energy among them.
No existing case sets `totalPower`. Outputs: battery and chargingstations.
