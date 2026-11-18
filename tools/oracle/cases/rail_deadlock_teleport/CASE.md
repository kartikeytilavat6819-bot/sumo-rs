# rail_deadlock_teleport

Closed single-track loop `A - B - C - D - A` with `rail_signal` junctions and bidi
edge pairs, so rail-signal driveways form a cycle. `clockwise` starts at t=0 and
`counter` at t=2 in the opposite direction; they meet head-on and the signal block
cycle cannot resolve, producing a rail-signal deadlock.

`--time-to-teleport.railsignal-deadlock 30` (with the general
`--time-to-teleport 120` kept longer, as C++ requires) detects and teleports a stuck
vehicle. No existing rail case sets this option.

Sim: `--end 300 --time-to-teleport 120 --time-to-teleport.railsignal-deadlock 30`.
