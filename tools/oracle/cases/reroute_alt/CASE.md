# Rerouting-with-an-alternative case

The existing `reroute/ref_device` has no alternative path, so rerouting can only fail there. This net has two
parallel routes between the same junctions, so a closure must produce a real detour.

- Net: `input.osm` — `10` (junction 1→2), then parallel `11` (top) and `12` (bottom) from 2→4, then `13` (4→6).
- Routes: `routes.rou` — `via_top` = `10 11 13`, three vehicles with `device.rerouting` (param form, period 5).
- Closure: `closing.add.xml` closes edge `11` for the whole run (rerouter on `10`, interval 0–200).
- Flags: `sim.args` = `--end 200 --additional-files tools/oracle/cases/reroute_alt/closing.add.xml`.

Expected C++ behaviour: vehicles detour onto `12`. Ours must do the same.
