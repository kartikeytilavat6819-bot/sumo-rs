# jt_no_internal — junction without internal links

Right-before-left 4-arm junction built with `--no-internal-links true`
(both `net.args` and `net_ref.sh`), so the net has only the eight external
edges and no `:C_*` internal edges. 20 vehicles, depart 0–54 s.

Tests the no-internal-links net layout and the resulting simulation. Net:
C++ netconvert 1.28.0; sim refs: C++ sumo 1.28.0, `--end 200`.
