# cv_teleport_remove
A vehicle stopped for 1000 s blocks a 1-lane road; d0's route `X E1` is disconnected (X and E1 both end at n2).
Base: `--time-to-teleport 20 --time-to-teleport.remove true --ignore-route-errors` (jammed vehicles are removed).
Variant `disc`: `--time-to-teleport.disconnected 3 --ignore-route-errors` (d0 teleports across the gap).
