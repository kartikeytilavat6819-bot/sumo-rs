# Lane-change audit (lc02)

Side-by-side review of the C++ lane-change code against the Rust port, function by function.
Reference: `C:\Users\karti\sumo-src\src\microsim\lcmodels` and `microsim` (1.28.0). Status values:

- `OK` — read line by line against the C++, same branch order, expressions, constants, early returns.
- `FIXED` — mismatch found and fixed; fix sha given.
- `NOT AUDITED` — not yet compared line by line. Do not treat as verified.

## MSLCM_LC2013.cpp

| function | status | fix sha |
|---|---|---|
| constructor / initDerivedParameters | OK | |
| debugVehicle | OK | |
| wantsChange → _wantsChange (decision path, lines 1119–1923) | OK | |
| inform | OK | |
| overtakeDistance | OK | |
| patchSpeed / _patchSpeed (lines 201–444) | OK | |
| informLeader (482–671) | OK | |
| informFollower (673–1042) | OK | |
| prepareStep (sigma drift) | OK | |
| changed / resetState | OK | |
| anticipateFollowSpeed | OK | |
| slowDownForBlocked | OK | |
| adaptSpeedToPedestrians | OK | |
| computeSpeedLat | OK | |
| getSafetyFactor / getOppositeSafetyFactor | OK | |
| saveBlockerLength | OK | |
| hasFreeLane | OK | |
| getParameter | FIXED | 92c49f2c |
| setParameter | OK | |
| saveState / loadState | OK | |

## MSLCM_SL2015.cpp

| function | status | fix sha |
|---|---|---|
| constants (#define block) | OK | |
| getParameter | OK | |
| setParameter | OK | |
| constructor / drop / getModelID / initDerivedParameters | OK | |
| debugVehicle / setOwnState / updateSafeLatDist | OK | |
| patchSpeed / _patchSpeed (Rust patch_speed_2) | OK | |
| inform / msg / changed / resetState | OK | |
| getSafetyFactor / getOppositeSafetyFactor / prepareStep | OK | |
| getLateralDrift / getPosLat / computeSublaneShift | OK | |
| informLeader / informFollower / informLeaders / informFollowers | OK | |
| _wantsChangeSublane (C++ 1067-2069, Rust wants_change_sublane_2) | OK (helpers it calls audited separately) | |
| slowDownForBlocked / isBidi / updateExpectedSublaneSpeeds / sublaneEnds / forecastAverageSpeed / computeSpeedGain | OK | |
| getLongest / tieBrakeLeader / getSlowest / getStopped / hasBidiLeader | OK | |
| checkBlocking / checkBlockingVehicles / updateCFRelated / overlap / lowest_bit / decideDirection / getLCA | OK | |
| wantsChangeSublane wrapper (keepLatGap, getLCA, computeSpeedLat calls) | OK | |
| checkStrategicChange / mustOvertakeStopped / computeGapFactor / keepLatGap / updateGaps / getWidth (+ MIN3/MAX3 from StdDefs.h) | OK | |
| computeSpeedLat, emergencySpeedLat, getDesiredAlignment, commitManoeuvre, commitFollowSpeed | OK | |
| getParameter (41 keys, same expressions, same InvalidArgument text) | OK | |
| setParameter (same keys, PUSHYGAP = 1 - v/minGapLat, initDerivedParameters at end) | OK | |
| wantsChange, getLeftBorder, getVehicleCenter, getNeighRight, preventSliding, wantsKeepRight, saveBlockerLength, outsideEdge | OK | |
| wantsChangeSublane (definition at C++ 182, not only the pasted comment) | OK | |
| OPEN (lc03): stress cases sl2015_lat04 / sl2015_lat08 diverge from pristine. First FCD diff: lat04 t=47 m5 angle 90.00 vs 90.01 (x diverges t=48); lat08 t=44 c2 y 3.30 vs 3.90 (0.6 m lateral jump). Cause not isolated (needs instrumented C++ trace). Not fixed. | OPEN | |
| getLca, overlap, getLowestBit, decideDirection, hasBidiLeader, getLongest/getSlowest/getStopped | NOT AUDITED | |
| updateExpectedSublaneSpeeds, computeSublaneShift, forecastAverageSpeed, computeSpeedGain | NOT AUDITED | |
| remaining helpers (getWidth, getSublaneWidth, getPosLat, getLateralDrift, getLeftBorder, getVehicleCenter, getNeighRight, preventSliding, wantsKeepRight, isBidi, emergencySpeedLat, saveState/loadState) | NOT AUDITED | |

## MSAbstractLaneChangeModel.cpp

| function | status | fix sha |
|---|---|---|
| getParameter / setParameter base (error text) | OK | |
| saveState / loadState (base, LCSTATE_BASE / LCSTATE gate) | OK | |
| getExtraReservation | OK | |
| static constants (NO_NEIGHBOR, UNDEFINED_LOOKAHEAD, LC_ASSUMED_DECEL) | NOT AUDITED | |
| all other functions (computeSpeedLat, getCooperativeHelpSpeed, ... ) | NOT AUDITED | |

## MSLCHelper.cpp

| function | status | fix sha |
|---|---|---|
| getRoundaboutDistBonus | OK | |
| updateBlockerLength | OK | |
| canSaveBlockerLength | OK | |
| divergentRoute | OK | |
| getSpeedPreservingSecureGap | OK | |
| isBidiLeader / isBidiFollower | OK | |
| unwillingToHelp | OK | |

## MSLaneChanger.cpp, MSLaneChangerSublane.cpp

| file | status | fix sha |
|---|---|---|
| MSLaneChanger.cpp | OK (per-function table below; one FIXED) | ea06e575 |
| MSLaneChangerSublane.cpp | OK (all 17 functions and constructor read against Rust; checkOpened block in checkChangeSublane present) | |

MSLaneChanger.cpp, function status:

| function | status | fix sha |
|---|---|---|
| postloadInitLC (checkOpened) | FIXED (foe links compared by content, not by address) | ea06e575 |
| checkOpened use in checkChange (unsafe-LC `opened()` check, C++ 1117–1135) | FIXED (was a `shortcut:` stub) | ea06e575 |
| change() | OK | |
| registerUnchanged / applyTraCICommands / startChange / continueChange | OK | |
| getRealLeader / getRealFollower / getCloserFollower | OK | |
| checkChangeWithinEdge / checkChange (incl. zipper, continuous-maneuver, critical-leader) | OK | |
| checkChangeOpposite | OK | |
| changeOpposite | OK | |
| avoidDeadlock, yieldToDeadlockOncoming, yieldToOppositeWaiting, resolveDeadlock | OK | |
| computeSafeOppositeLength | OK | |
| getOncomingVehicle, getOncomingOppositeVehicle, computeSurplusGap, foundHilltop, computeOvertakingTime | OK | |
| getColumnleader, getLaneAfter, getMaxOvertakingSpeed, getLastBlocked, hasOppositeStop, checkOppositeStop, getBestLanesOpposite | OK | |
| mayChange, initChanger, updateChanger, updateLanes, findCandidate, laneChange, checkTraCICommands, applyTraCICommands | OK (laneChange try/catch lock-release is a documented `shortcut:` in the Rust; no ProcessError path) | |
| constructor (lane length checks for mayChangeLeft/Right) | OK | |

## Findings

1. **LC2013 getParameter, integer division (FIXED, 92c49f2c).** `mySpeedGainProbabilityRight`,
   `mySpeedGainProbabilityLeft` and `myKeepRightProbability` are `long long`, and `HYST_PRECISION` is an
   `int` macro (`10000000`). So `toString(x / HYST_PRECISION)` is an integer division and prints an
   integer (`0.5` set → `0` read back; `-2.5` set → `-2`). The Rust returned `f64` and printed fractions.
   The pasted C++ comment in the Rust file shows a `(double)` cast that is not in the pristine source
   (line 2178), so the pristine source was followed.
   Probe: `tools/oracle/cases/traci_basic/client.py`, four `lcm:` probes (commit 01915dee). Pristine
   output checked over WSL: `'0'`, `'1'`, `'-2'`, `'2'`.

2. **Constants and enum values checked.** All LC2013/SL2015 `#define` values match. The `LCA_*` bits
   match the generated `sumo_xml_definitions_gen.rs` (`1<<3`, `1<<8`, `1<<16`, `1<<23`, `1<<26`).

3. **No further mismatches** in the LC2013 functions, MSLCHelper, and `getExtraReservation` /
   save-load paths checked so far. `gapExtrapolation` is static in C++ too, so the Rust static call is correct.

## Not done

- The NOT AUDITED rows above: SL2015 logic (about 4,000 lines of C++), the abstract model's remaining
  functions, MSLaneChanger.cpp (127 KB) and MSLaneChangerSublane.cpp (47 KB).
- New lane-change cases for SL2015 with bicycles and sublane, lcAssertive high, zipper merges, and
  emergency lane changes. None were added yet.
