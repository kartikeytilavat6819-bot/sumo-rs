# rr_weight_files

Online rerouting seeded from a `meandata` weight file: `--weight-files` plus
`--device.rerouting.init-with-loaded-weights`. The loaded weights make edge `11` much slower
(50 s) than the parallel `12` (12 s), so the period-10 rerouting device (probability 1) should
switch vehicles onto `12` without any closure. Refs generated with pristine C++ 1.28.0 (WSL).
