# bike_indirect_left

Two-stage (indirect) bicycle left turns at a signalized four-arm junction.
The four left-turn connections are marked `indirect="true"` in the connection
file so netconvert bakes the indirect-left geometry and TLS link states into
`ref_simple.net.xml`; oncoming straight cars create the gaps.

- Net: plain XML node/edge/connection files, see `net.args`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_indirect_left`.
