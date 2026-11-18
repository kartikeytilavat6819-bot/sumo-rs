# hw_jam_teleport

Highway bottleneck: two-lane E0 (400 m) narrows to one-lane E1 (600 m) at 25 m/s. A bus stops
at E1 300 m for 200 s while a flow of 150 cars (period 2 s) piles up. Teleports are enabled for
both normal roads and highways at 60 s (`--time-to-teleport 60 --time-to-teleport.highways 60`).

Sim: `--end 300`. Net from node/edge XML via `net_ref.sh`.
