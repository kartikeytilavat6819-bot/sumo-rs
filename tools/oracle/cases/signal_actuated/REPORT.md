# signal_actuated: PASS

- Horizon check: the C++ actuated run at `--end 200` differs from the C++ static run on the same net and routes (11 FCD lines differ), so the logic changes phases in this window.
- run_all: net PASS, sim PASS (tripinfo, fcd, statistics) against C++ sumo.
- Before the fix: `options_cont` panic (`tls.actuated.jam-threshold` and `detector-length` are float options read with `get_string`), then a detector panic in `MSDetectorFileOutput::writeXMLDetectorProlog`.
