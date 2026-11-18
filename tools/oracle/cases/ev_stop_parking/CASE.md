# Case `ev_stop_parking`: parking stop at a charging station

Two EVs stop at `cs0` for 40 s, one with `parking="1"` (offroad parking) and one with the default onroad stop.
`MSVehicle::isParking()` clamps battery consumption to <= 0 while parked and the charging station's
`enter()` receives the offroad flag. Existing charging stops never set `parking`. Outputs: battery and
chargingstations.
