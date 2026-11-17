# cv_calibrator_probe
Edge calibrator on E1 whose flows have no route: inserted vehicles sample routes from routeProbe rp0 (on E0).
Intervals: 0-100 s 600 veh/h (below demand: removals), 100-200 s 2400 veh/h (insertions), 200-300 s speed-only;
jamThreshold 0.6. routeProbe output goes to /dev/null.
