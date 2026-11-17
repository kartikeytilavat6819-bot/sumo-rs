# Case `device_toc`: Takeover (ToC) device

Net: `stops/ref_simple.net.xml`. `--device.toc.probability 1` equips every vehicle; `device.toc.file` (via
`extra_outputs.txt`) is compared with `ref.toc.xml`. `blocker` (manual, stopped) forces a dynamic ToC on `toc_dyn`
(`dynamicToCThreshold 3`, `dynamicMRMProbability 0.5`). `toc_manual` starts in manual mode.
Reference: C++ sumo 1.28.0 from `~/sumo-pristine` via `make_sim_refs.sh device_toc`.
