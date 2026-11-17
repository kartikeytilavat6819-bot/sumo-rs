# reroute_bypass_p1 — the brief's `period=1` configuration (expected FAIL)

Same net and closure as `../reroute_bypass` (`ref_simple.net.xml` copied from there), with the routing device at
`device.rerouting.period=1` exactly as specified.

- C++ 1.28.0: `rerouteNo=2`, vehicle stays on `AB`, arrival 53.00. C++ reverts the closure detour, so this config
  cannot test a detour at all.
- Ours: keeps the bypass (`AC`/`CB`), arrival 61.00. First difference FCD `t=20`, `v0` `x` 286.55 vs 286.38.

This FAIL is recorded on purpose. See `../reroute_bypass/REPORT.md`.
