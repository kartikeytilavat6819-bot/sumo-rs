# rail_veh_output

`--railsignal-vehicle-output` alone on a rail-signal net with a passing loop and four
trains. Existing `out_railsignal` and `rail_block_output` write this output together
with `--railsignal-block-output`; this case isolates the vehicle-output option so
a regression in it is not masked by the block output.

Compared by run_all: tripinfo, fcd, stat and `ref.vehicles.xml` (compare_xml.py, exact).

Sim: `--end 300 --time-to-teleport -1`.
