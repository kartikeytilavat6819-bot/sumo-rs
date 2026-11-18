# hw_truck_keepright

Three-lane highway (E0, 1000 m, 25 m/s). Slow trucks (`vClass="truck"`, length 12 m) start in
lanes 1 and 2 with a strong `lcKeepRight="3"`; fast cars start in lane 0. Keep-right must push
the trucks back to the right while cars pass on the left.

Sim: `--end 200`. Net from node/edge XML via `net_ref.sh`.
