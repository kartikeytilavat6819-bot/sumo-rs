# lc_dk2008

Three-lane E0 E1 with an on-ramp `ramp` (1 lane) joining E1 at n1, no sublanes. Slow leaders (4 m/s) on lanes 0 and 1.
Three `DK2008` vehicle types (lcA, lcB, lcC) carry the same lc* attributes as the other cases. The C++ DK2008
constructor reads none of them itself; only the abstract base reads lcSigma, lcAssertive, lcOvertakeRight,
lcMaxSpeedLatStanding and lcMaxSpeedLatFactor (plus the cooperative-help values), so the case covers those through the base class. Ramp vehicles use route `rr`.
Sim: `--end 200`. Reference: C++ sumo 1.28.0 from ~/sumo-pristine.
