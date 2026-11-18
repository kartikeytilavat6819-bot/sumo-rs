# hw_vss_segment

Three-lane highway (E0 -> E1, 25 m/s). Three variable speed signs at 200 m on E0 impose
different speed limits per lane (10 / 15 / 20 m/s) that change over time (40 s, 80 s steps).
Cars start in all lanes and react to the per-lane limits, with lane changes.

Sim: `--end 200 --additional-files tools/oracle/cases/hw_vss_segment/hw_vss_segment.add.xml`.
Net from node/edge XML via `net_ref.sh`.
