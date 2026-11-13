# Oracle harness validation

Validated on branch `worktree-agent-vh01` against real C++ SUMO v1.28.0 output (WSL build).
Scripts were not modified.

## 1. Positive: compare_net.py ref vs ref

Command: `python tools/oracle/compare_net.py tools/oracle/cases/<c>/ref.net.xml tools/oracle/cases/<c>/ref.net.xml`

| case | exit | result |
|---|---|---|
| junction | 0 | 0 differences (216 elements) |
| roundabout | 0 | 0 differences (76 elements) |
| signal | 0 | 0 differences (75 elements) |

## 2. Negative: compare_net.py ref vs ref with one lane speed changed

Mutated copies were written to `%LOCALAPPDATA%\Temp\opencode\<c>_neg.net.xml` by `mutate.py`
(first `<lane ... speed=...>` set to `1.23`). Not committed.

Command: `python tools/oracle/compare_net.py tools/oracle/cases/<c>/ref.net.xml <mutated>`

| case | changed lane | exit | report |
|---|---|---|---|
| junction | `:-137712_0_0` (3.65 -> 1.23) | 1 | `lane ':-137712_0_0' speed: ref='3.65' cand='1.23'` |
| roundabout | `:2289279154_0_0` (3.65 -> 1.23) | 1 | `lane ':2289279154_0_0' speed: ref='3.65' cand='1.23'` |
| signal | `:1802245009_0_0` (6.59 -> 1.23) | 1 | `lane ':1802245009_0_0' speed: ref='6.59' cand='1.23'` |

## 3. Simulation stage: C++ sumo reference tripinfo

Command (run per case, `<c>` = junction, roundabout, signal):

```
wsl -e bash -lc 'SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n /mnt/c/Users/karti/sumo-rs/.claude/worktrees/agent-vh01/tools/oracle/cases/<c>/ref.net.xml -r /mnt/c/Users/karti/sumo-rs/.claude/worktrees/agent-vh01/tools/oracle/cases/<c>/routes.rou --tripinfo-output /tmp/ti_<c>.xml --no-step-log true --duration-log.statistics false'
```

All three runs completed and wrote `/tmp/ti_<c>.xml` (about 2.9 kB each). Copied to
`tools/oracle/cases/<c>/ref.tripinfo.xml`.

Note: the PowerShell `$HOME` must be single-quoted, otherwise it expands before WSL sees it.

## 4. compare_sim.py

| command | case | exit | output |
|---|---|---|---|
| `--tripinfo ref ref` | junction | 0 | MATCH |
| `--tripinfo ref ref` | roundabout | 0 | MATCH |
| `--tripinfo ref ref` | signal | 0 | MATCH |
| `--tripinfo ref mutated` (duration of first trip -> 99.5) | junction | 1 | `vehicle v1 field duration: ref='13.00' cand='99.5'` |
| `--tripinfo ref mutated` (duration of first trip -> 99.5) | roundabout | 1 | `vehicle v0 field duration: ref='21.00' cand='99.5'` |
| `--tripinfo ref mutated` (duration of first trip -> 99.5) | signal | 1 | `vehicle v0 field duration: ref='10.00' cand='99.5'` |

## 5. Unit tests

`python -m unittest discover -s tools/oracle -p "test_*.py"`: 10 tests, all pass.

## Script issues found (not fixed)

1. `compare_sim.py`, `compare_tripinfo()` (lines 47-48): `{e.get("id"): ...}` silently keeps the last
   entry when a vehicle id repeats. A duplicate id in either file is masked, not reported.
   `compare_net.py` handles duplicates with `add()`; `compare_sim.py` does not.
2. `compare_net.py`, `collect()` (lines 54-71): only `junction`, `node`, `crossing`, `walkingArea`,
   `tlLogic`, `edge`/`lane`, and `connection` are compared. Top-level `<location>`, `<prohibition>`,
   and `<roundabout>` are silently ignored, so differences there go undetected. The signal ref
   contains `<location>`, which is not checked.
3. No functional bugs found in the comparison logic. Positive, negative, and simulation checks
   all behaved as specified.

## Not done / caveats

- Only lane `speed` was mutated for the net negative test, and only `duration` for the sim test.
- The mutated copies and the `/tmp` tripinfo files live outside the repo.
