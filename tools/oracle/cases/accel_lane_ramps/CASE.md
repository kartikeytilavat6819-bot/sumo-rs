# accel_lane_ramps

Plain-XML freeway with an on-ramp; netconvert `--ramps.guess true` splits H1 and inserts
`H1-AddedOnRampEdge` with an `acceleration="1"` lane. Highway traffic uses
`H0 H1-AddedOnRampEdge H1 H2`, ramp traffic joins via `R0 H1-AddedOnRampEdge H1 H2`; FCD lanes on the
added edge show ramp traffic merging from the acceleration lane. Sim: `--end 150`.