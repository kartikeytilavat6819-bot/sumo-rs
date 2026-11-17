# fcdattrs

Two-lane 500 m edge `A`. Vehicles `v0`, `v1`, `v2` and flows `fLine`, `fPlain` set `color` and `line` (`v1` on lane 1, `fLine` on lane 1).

- `ref` (`sim.args`): `--end 90 --fcd-output.attributes speed,angle,lane,type,acceleration,location`, which restricts the FCD output to those attributes.
- `ref.vehroute` (`extra_outputs.txt`): `--vehroute-output`, compared as XML, so `line` / `color` handling is checked where C++ writes it.

Reference: C++ sumo 1.28.0 from ~/sumo-pristine (`net_ref.sh`, `make_sim_refs.sh`).
