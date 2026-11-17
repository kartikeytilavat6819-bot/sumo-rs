# bluelight — FAIL (ours panics)

C++ reference (`make_sim_refs.sh bluelight`, pristine 1.28.0), ours from the VM gate build of this
branch (`run_all.sh`):

```
bluelight/ref  sim-PANIC  panicked at crates/sumo_core/src/microsim/ms_vehicle_type.rs:1464:13:
                          could not add singular type car@c0
```
preceded by our warning (C++ swallows this in `MSDevice_Bluelight::notifyMove`):

```
Setting parameter 'minGapLat' is not supported for laneChangeModel of type 'LC2013'
```

First differing quantity: `c1` is missing from our tripinfo (we die at t≈60 while `em0` is 25 m
behind `c0`, the first step where the device tries to set the rescue-lane lane-change parameters).
C++ numbers: `c0` 124 s / `c1` 122 s / `em0` 126 s (`timeLoss` 17.40), `em1` 178 s (`timeLoss` 0.00).

Cause to look at: `MSDevice_Bluelight` legitimately sets `minGapLat` (C++ catches `InvalidArgument`
only for models without it and ignores failures); our `MSVehicleType` singular-type creation aborts
instead of absorbing the unsupported-parameter path, so `setPreferredLateralAlignment` /
`setMinGap` on the singular type never completes.
