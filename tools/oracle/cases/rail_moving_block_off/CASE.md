# rail_moving_block_off

Same single-track-with-passing-loop net as `rail` / `rail_moving_block`, with four
trains (two per direction). Unlike `rail_moving_block` (which sets
`--railsignal-moving-block true`), this case sets `--railsignal-moving-block false`
explicitly, exercising the fixed-block branch of rail-signal driveway operation.

Sim: `--end 300 --time-to-teleport -1 --railsignal-moving-block false`.
