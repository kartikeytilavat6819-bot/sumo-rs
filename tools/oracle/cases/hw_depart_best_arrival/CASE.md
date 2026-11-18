# hw_depart_best_arrival

Four-lane highway (E0 -> E1 -> E2, 25 m/s) with a two-lane off-ramp (off, 15 m/s) at n2. A slow
blocker starts in lane 0; through cars use `departLane="best"` and pick another lane. Exiting
cars end on the off-ramp with `arrivalLane` (0 / 1) and `arrivalSpeed="5.00"`.

Sim: `--end 200`. Net from node/edge XML via `net_ref.sh`.
