# jt_pass — connections with `pass="true"`

Priority 4-arm junction; the two major-road through connections
`N_in -> S_out` and `S_in -> N_out` carry `pass="true"` (serialised as
`pass="1"` in the net), so those movements do not yield to foes. The other
connections keep default yielding. 20 vehicles, depart 0–54 s.

No committed case exercises `pass` connections. Net: C++ netconvert 1.28.0;
sim refs: C++ sumo 1.28.0, `--end 200`.
