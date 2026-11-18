# cfg_fcd_signals_accel

FCD output sub-options `--fcd-output.signals true` and `--fcd-output.acceleration true`, which add the `signals` (brake/blinker state) and `acceleration` attributes to every `<vehicle>` in fcd output. Two-lane 500 m edge `A` with a slow leader so acceleration is non-zero.

Sim: `--end 90 --fcd-output.signals true --fcd-output.acceleration true`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_fcd_signals_accel; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
