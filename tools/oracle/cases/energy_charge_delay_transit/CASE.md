# Case `energy_charge_delay_transit`: charging station with chargeDelay, chargeInTransit, efficiency

`chargingStation cs_transit` on lane -103224#2_0: `chargeInTransit="true" chargeDelay="5" efficiency="0.8"`.
routes.rou: v0 stops at the station for 30 s (energy charged 355.56 Wh over 42 charging steps in C++); other vehicles drive through.
Outputs: `--battery-output battery --chargingstations-output chargingstations`.
