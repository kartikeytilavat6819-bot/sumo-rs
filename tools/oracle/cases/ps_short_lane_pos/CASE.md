# ps_short_lane_pos

Stops on a deliberately short 50 m edge `E1`, using explicit `startPos`/`endPos` inside the lane (with and
without `startPos`) to test stop placement and blocking on short lanes. A through vehicle is held up.

Net: `E0`(120 m) → `E1`(50 m) → `E2`(130 m), all 1 lane, 13.89 m/s. Sim: `--end 200`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_short_lane_pos; rm NEW_CASE`.
