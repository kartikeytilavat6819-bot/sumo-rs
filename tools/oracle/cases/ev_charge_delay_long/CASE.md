# Case `ev_charge_delay_long`: chargeDelay longer than a stop

`chargingStation cs_slow` has `chargeDelay="60"`. `short_stop` dwells only 20 s, so it never passes the
delay and receives no energy; `long_stop` dwells 90 s and starts charging after the 60 s delay. Existing
delay cases use 3-8 s delays with 25-30 s stops, so they always charge. Outputs: battery and chargingstations.
