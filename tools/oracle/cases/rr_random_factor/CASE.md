# rr_random_factor

Rerouting with randomly disturbed edge weights: `--weights.random-factor 2` draws a uniform
factor in [1,2) per edge/vehicle during routing. On the parallel-edge net (equal-length `11`/`12`)
this makes periodic rerouting (probability 1, period 10) pick different branches. Fixed `--seed 5`
for reproducibility. No closure. Refs generated with pristine C++ 1.28.0 (WSL).
