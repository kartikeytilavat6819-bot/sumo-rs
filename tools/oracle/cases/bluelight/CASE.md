# bluelight

Emergency vehicles with `has.bluelight.device` at a static traffic light.

Net: `n0 → n1 → n2`, `n1` is a `traffic_light` (netconvert generates the static 82 s green / 3 s yellow /
5 s red program that `ref_simple.net.xml` shows); `E0`/`E1` northbound (2 lanes), `R0`/`R1` southbound.

`em0` (vClass `emergency`, 25 m/s, `sigma=0`, `param has.bluelight.device=true`) departs at t=60 and
reaches the stop line at about t=86, i.e. in the red phase, behind the queued cars `c0`…`c4`. `em1`
departs at t=130 and crosses on green as a control. Opposite direction: `d0`, `d1`.

The measured hook is the tripinfo `devices` attribute: `devices="tripinfo_em0 bluelight_em0 fcd_em0"`.
A device that is configured but never built leaves every number identical, so that attribute is the
proof it exists. The rescue-lane / minGap side effects then show up in the cars' `timeLoss`.

Sim: `--end 200`.
Refs: `make_sim_refs.sh bluelight` (pristine C++ 1.28.0), net via `net_ref.sh`.
