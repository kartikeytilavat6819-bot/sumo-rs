# lc_sl2015

Three-lane E0 E1 with an on-ramp `ramp` (1 lane) joining E1 at n1, run with `--lateral-resolution 0.8` (sublanes).
Slow leaders (4 m/s) on lanes 0 and 1. Three `SL2015` vehicle types (lcA, lcB, lcC) vary the SL2015 parameters:
lcStrategic, lcCooperative, lcSpeedGain, lcKeepRight, lcOpposite, lcLookaheadLeft, lcSpeedGainRight, lcSublane,
lcPushy, lcPushyGap, lcAssertive, lcImpatience, lcTimeToImpatience, lcAccelLat, lcSigma, lcMaxSpeedLatStanding,
lcMaxSpeedLatFactor, lcTurnAlignmentDistance, lcLaneDiscipline, lcOvertakeRight, lcCooperativeRoundabout,
lcCooperativeSpeed, lcOvertakeDeltaSpeedFactor. Ramp vehicles use route `rr`.
Sim: `--end 200 --lateral-resolution 0.8`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine.
