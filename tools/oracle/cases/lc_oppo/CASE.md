# lc_oppo

Opposite overtaking: three-lane E0 E1 with reverse three-lane E0r E1r (two-way road), plus an on-ramp `ramp`
joining E1 at n1. Slow leader `s0` (4 m/s) on lane 1 of E0 and slow oncoming `o0` (route `oppo`, E1r E0r).
LC2013 types vary lcOpposite (0, 0.5, 1, 2), lcKeepRight, lcStrategic, lcSpeedGain and lcOvertakeRight, so ego
vehicles can overtake on the opposite lanes. Ramp vehicles use route `rr`.
Sim: `--end 200`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine.
