# jt_impatience — `--time-to-impatience` at a priority junction

Priority 4-arm junction, 20 vehicles (depart 0–54 s), sim with
`--end 200 --time-to-impatience 10` (default is 30 s). Minor-road vehicles
become impatient after 10 s of waiting and force their way earlier. The
committed `ignore_blocker_impatience` case combines this with
`--ignore-junction-blocker`; this isolates `--time-to-impatience`.

Net: C++ netconvert 1.28.0; sim refs: C++ sumo 1.28.0.
