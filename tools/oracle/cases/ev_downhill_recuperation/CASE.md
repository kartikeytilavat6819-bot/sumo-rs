# Case `ev_downhill_recuperation`: recuperation on a downhill net built with netconvert elevation

Net built from plain node/edge XML via `netconvert` with `z` elevations 100 -> 60 -> 20 m over 800 m, so
the EV descends a 10% slope. `Energy/unknown` vType with `recuperationEfficiency="0.9"`, battery device
30 kWh / 10 kWh initial charge. On the downhill the energy model yields negative consumption, so the
battery output shows energy being regenerated. `net.args` / `net_ref.sh` regenerate the net with C++
netconvert. Covers the slope/recuperation path (no existing case has an EV on an elevated net).
