# rr_period

Basic runtime rerouting device: `--device.rerouting.probability 1 --device.rerouting.period 10`
on the small parallel-edge net from `reroute_alt` (edges 10 -> {11 top,12 bottom} -> 13).
A rerouter closes the preferred top edge `11` for the whole run, so the period-10 rerouting
device must move the 5 vehicles onto `12`. Refs generated with pristine C++ 1.28.0 (WSL).
