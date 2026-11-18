# taxi_only_lane

Taxi-only lane: E0 lane 0 `allow="taxi"`, lane 1 `allow="passenger"`. Taxis and cars share the corridor;
two use `departLane="allowed"` so only the permitted lane is selectable. FCD lanes show taxis on lane 0 and
cars on lane 1. Sim: `--end 120`.