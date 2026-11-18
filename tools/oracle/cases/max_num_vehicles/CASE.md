# max_num_vehicles

`--max-num-vehicles 2` delays insertion so at most two vehicles are in the simulation at
once. Eight vehicles all want to depart at t=0 on a 1-lane road, so the inserter holds the
rest back and admits them as earlier ones leave.

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from `collision`).

Sim: `--end 200 --max-num-vehicles 2`.
Compared: tripinfo (depart times / departDelay), fcd and stat (`vehicles loaded/inserted/running`).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh max_num_vehicles; rm NEW_CASE`.
