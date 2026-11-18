# rail_max_block

Same rail-signal net with passing loop as `rail`, but `--railsignal.max-block-length 200`
limits driveways to 200 m. The 700 m section between signals `r1` and `r2` exceeds the
limit, so C++ warns and does not build the long block; three trains (two east, one west)
then run with the shortened/absent blocks. This exercises the
`--railsignal.max-block-length` option, which no existing rail case sets.

Sim: `--end 300 --time-to-teleport -1 --railsignal.max-block-length 200`.
