# bike_priority_left

Bicycles making left turns at an unsignalized priority junction while
straight-going cars approach from the conflicting arm. Tests bicycle yielding
at a priority junction (no TLS) and the left-turn internal-link path; FCD
positions and tripinfo waiting times are the hooks.

- Net: plain XML node/edge/connection files, see `net.args`.
- Sim: `--end 120`. Refs: `make_sim_refs.sh bike_priority_left`.
