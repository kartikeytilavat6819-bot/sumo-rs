# cfg_sumocfg

Loading options from a `.sumocfg` configuration file via `-c` in `sim.args`. The config sets `end`, `precision 4` and `time-to-impatience`; net/route/output files still come from run_all's command line (SUMO reparses the command line after loading the config, so those take precedence). The four-decimal numbers in the outputs prove the config was read.

Sim: `-c tools/oracle/cases/cfg_sumocfg/cfg.sumocfg --end 80`.

Compared by run_all: tripinfo, fcd and stat (exact). Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh cfg_sumocfg; rm NEW_CASE` (pristine C++ 1.28.0, WSL).
