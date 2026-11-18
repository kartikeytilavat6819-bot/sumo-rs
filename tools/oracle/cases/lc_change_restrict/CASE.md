# lc_change_restrict

Per-lane `changeLeft` / `changeRight` permissions on E0 (3 lanes): lane 0 `changeLeft="bus"` forbids cars
from changing left out of lane 0; lane 2 `changeRight="truck"` restricts right changes. Slow truck blockers
sit on lanes 0 and 2; cars behind them must stay or change lane depending on the permission, while a bus on
lane 0 can change left. FCD lane traces are the hook. Sim: `--end 150`.