# lc_dk2008

Three-lane E0 E1 with an on-ramp `ramp` (1 lane) joining E1 at n1, no sublanes. Slow leaders (4 m/s) on lanes 0 and 1.
Three `DK2008` vehicle types (lcA, lcB, lcC) vary the lc parameters (lcStrategic, lcCooperative, lcSpeedGain, lcKeepRight,
lcOpposite, lcLookaheadLeft, lcSpeedGainRight, lcAssertive, lcImpatience, lcTimeToImpatience, lcAccelLat, lcSigma,
lcMaxSpeedLatStanding, lcMaxSpeedLatFactor, lcTurnAlignmentDistance, lcLaneDiscipline, lcOvertakeRight,
lcCooperativeRoundabout, lcCooperativeSpeed, lcOvertakeDeltaSpeedFactor). Ramp vehicles use route `rr`.
Sim: `--end 200`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine.
