# rr_tls_mode

Rerouting at a signalised 4-arm junction (net from `tls_delay_loops`, static-looking delay-based
TLS at `x`): `--device.rerouting.mode 1` uses aggregated travel times (which include signal delay)
and `--weights.tls-penalty 5` adds a green-split-based cost on TLS internal lanes. A rerouter closes
outgoing edge `E0`, so period-10 rerouting sends vehicles from `W0`/`S0` to `N0`.
Refs generated with pristine C++ 1.28.0 (WSL).
