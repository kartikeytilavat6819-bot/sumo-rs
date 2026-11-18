# jt_jam_teleport — jammed priority junction with `--time-to-teleport 20`

A `blocker` stops on `S_out_0` for 500 s, blocking the major road; 16 major
and 8 minor vehicles queue behind it and jam the junction. Sim args:
`--end 200 --time-to-teleport 20`. The reference statistics report 9 jam
teleports, so the case exercises junction blocking plus teleport handling.

Net: C++ netconvert 1.28.0; sim refs: C++ sumo 1.28.0.
