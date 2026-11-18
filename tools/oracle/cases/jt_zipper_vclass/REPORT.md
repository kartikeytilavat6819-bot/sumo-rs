# jt_zipper_vclass — 3-lane zipper merge with mixed vehicle classes

`type="zipper"` node `nZ`: 3-lane `IN` (speed 13.89) merges to 1-lane `OUT`.
40 vehicles depart 0–19.5 s at 0.5 s headways, cycling through `passenger`,
`bicycle`, `motorcycle` and `truck` vTypes. The committed `zipper` case is a
2-to-1 merge with default cars; this stresses zipper alternation with three
lanes and differing vehicle classes.

Net: C++ netconvert 1.28.0; sim refs: C++ sumo 1.28.0, `--end 200`.
