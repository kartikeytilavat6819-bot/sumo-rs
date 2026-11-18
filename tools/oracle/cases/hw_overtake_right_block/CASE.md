# hw_overtake_right_block

Three-lane highway (E0, 1000 m, 25 m/s). Slow cars occupy lanes 1 and 2 (the two left lanes);
fast cars start in lane 0 (right). With `--lanechange.overtake-right true`, the fast cars can
overtake the slow block on the right.

Sim: `--end 200 --lanechange.overtake-right true`. Net from node/edge XML via `net_ref.sh`.
