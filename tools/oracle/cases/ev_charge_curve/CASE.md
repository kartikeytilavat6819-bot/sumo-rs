# Case `ev_charge_curve`: battery `device.battery.chargeLevelTable` / `chargeCurveTable`

The battery device accepts a piecewise-linear charge curve as two whitespace-separated tables. Here the maximum
charge rate falls from 15000 W at low state of charge to 500 W when full, so a 60 s stop at a 100 kW station
charges much less than the nominal station power. No existing case uses these two parameters. Outputs:
battery and chargingstations.
