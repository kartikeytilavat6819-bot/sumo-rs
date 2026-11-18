# disallow_truck_lane

E0 lane 0 uses a disallow list (`disallow="truck"`) instead of an allow list; lane 1 also has a lower
per-lane speed. Trucks must use lane 1, cars prefer lane 0; one truck uses `departLane="allowed"`.
Sim: `--end 120`.