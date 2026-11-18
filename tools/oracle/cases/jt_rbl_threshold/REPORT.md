# jt_rbl_threshold — `--junctions.right-before-left.speed-threshold`

Right-before-left 4-arm junction with all edges at 5.0 m/s, built with
`--junctions.right-before-left.speed-threshold 2.0` (in `net.args` and
`net_ref.sh`). The option is serialised into the net's `<junctions>` block;
netconvert must parse and echo it. 20 vehicles, depart 0–54 s.

Note: `--junctions.minimal-link-speed` does not exist in SUMO 1.28.0, so this
valid junction option is used instead. Net: C++ netconvert 1.28.0; sim refs:
C++ sumo 1.28.0, `--end 200`.
