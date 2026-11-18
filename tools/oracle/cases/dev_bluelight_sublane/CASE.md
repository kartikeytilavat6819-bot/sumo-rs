# dev_bluelight_sublane

Bluelight device (emergency vehicles, reactiondist 60) in a sublane simulation (lateral-resolution 0.8) on the 2-lane junction net with dense traffic.

Sim: `--end 250 --lateral-resolution 0.8 --device.bluelight.reactiondist 60`.

Compared by run_all: tripinfo, fcd, stat.
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh dev_bluelight_sublane; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
