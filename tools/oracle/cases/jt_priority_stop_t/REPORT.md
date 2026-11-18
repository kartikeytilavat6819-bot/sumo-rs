# jt_priority_stop_t — 3-arm (T) `priority_stop` junction

Centre node `C` type `priority_stop`, arms N/E/S with N/S major (priority 2)
and E minor (priority 1). 20 vehicles, depart 0–54 s. The committed
`jt_priority_stop` uses four arms; this tests the stop rule on a T junction.

Net: C++ netconvert 1.28.0; sim refs: C++ sumo 1.28.0, `--end 200`.
