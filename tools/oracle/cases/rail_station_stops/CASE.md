# rail_station_stops

Single-track line `w - r1 - r2 - e` with `busStop`-based train stations declared in
`station.add.xml`: `central` and `east` on the eastbound lanes, `central_w` on the
westbound lane. `t_east` stops at `central` with `until="60"` and at `east` with
`duration="20"`; `t_west` stops at `central_w` with `until="75"`. This exercises
station (busStop) stops combined with absolute `until` and relative `duration`,
which the existing `rail_stops_until` case does not (it stops on bare lanes).

Sim: `--end 300 --time-to-teleport -1 --additional-files .../station.add.xml`.
