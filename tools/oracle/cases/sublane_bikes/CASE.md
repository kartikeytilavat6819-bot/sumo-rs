# sublane_bikes

Sublane model at `--lateral-resolution 0.4` on a 2-lane, 400 m edge `E0`, with bicycles sharing the lanes
with passenger cars.

- Bicycles `bike0..bike3` (`vClass=bicycle`, 0.6–0.8 m wide, 4.5–5 m/s) and cars `car0..car4` (1.8 m) plus slow cars `slow0`, `slow1` (4 m/s).
- Sim: `--end 200 --lateral-resolution 0.4`. Reference: `make_sim_refs.sh sublane_bikes` (pristine C++ 1.28.0).
- Net: `net_ref.sh` (netconvert from the two plain XML files).
