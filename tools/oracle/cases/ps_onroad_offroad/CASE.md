# ps_onroad_offroad

Direct comparison of `onRoad="true"` parking (vehicle stays on the lane and blocks following traffic)
against default off-road roadside parking, both on the two-lane edge `E1`. Two vehicles park off-road on
`E1_1` and two park on-road on `E1_0`; a through flow shows the blocking difference.

Net: `E0`(1 lane) → `E1`(2 lanes) → `E2`(1 lane), 300 m each. Sim: `--end 200`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_onroad_offroad; rm NEW_CASE`.
