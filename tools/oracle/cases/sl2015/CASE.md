# sl2015

Two-lane E0 E1. Vehicle type `sl` uses `laneChangeModel="SL2015"` (no `--lateral-resolution`, so no sublanes).
Two slow leaders (4 m/s) on lane 0 and mixed `lc2013` cars.
Sim: `--end 300`. Only one vehicle arrives by t=300 in the C++ reference; the FCD trajectories carry most of the signal.
