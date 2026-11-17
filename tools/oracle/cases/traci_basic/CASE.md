# traci_basic

TraCI (live control) oracle. Unlike the other cases this does not compare output files: it starts
`sumo` with `--remote-port`, connects with the Python `traci` client (1.28.0), runs a fixed command
sequence, and compares every answer (value, Python type, exception text) between our `sumo.exe` and
C++ SUMO 1.28.0 running in WSL.

Cases: `junction` (vehicles, lanes, the setter; no traffic light) and `signal` (the traffic light
`1802245009` for `trafficlight.getRedYellowGreenState`). They reuse the nets and routes in
`../junction` and `../signal`.

Run:

```bash
python tools/oracle/cases/traci_basic/client.py               # junction and signal, both sides
python tools/oracle/cases/traci_basic/client.py --case signal --only ours
```

Our binary defaults to `C:/Users/karti/sumo-rs-target/debug/sumo.exe` (`--ours` overrides). The C++
side runs as `$HOME/sumo-src/bin/sumo` inside WSL with `SUMO_HOME=$HOME/sumo-src`.

Not covered: subscriptions, `changeLane`, `trafficlight.setRedYellowGreenState`, multi-client ordering,
`simulation.load`, and any command other than the ones listed in `client.py`.
