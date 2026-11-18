# buslane_allow

Bus lane on E0: lane 0 `allow="bus"`, lane 1 `allow="passenger"`. Buses and cars depart in mixed order;
two use `departLane="allowed"` to check permission-aware lane selection. FCD `lane` shows that buses pick
lane 0 and cars pick lane 1. Sim: `--end 120`.