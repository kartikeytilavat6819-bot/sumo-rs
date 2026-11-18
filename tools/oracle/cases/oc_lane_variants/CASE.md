# oc_lane_variants

Lane-selection values not covered elsewhere: `departLane="best_prob"` (probabilistic best-free),
plus `arrivalLane="first"` / `"random"` on a 3-lane edge whose lane 2 is delivery-only.
Two seeds check the random variants. Sim: `--end 90`, seeds 5 and 11.
