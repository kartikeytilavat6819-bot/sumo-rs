# Lane-change — verdict (2026-11-16)

| Stage | Verdict | Detail |
|---|---|---|
| net | PASS | exact |
| sim (`ref`) | **PASS** | tripinfo MATCH, fcd MATCH, stat 0 differences |

Lane changes exercised (C++ FCD, confirmed ours identical): `blocker` 100_1 → 100_0 at t=7.00.
5 of 6 vehicles arrive by t=120 in the reference. The `chaser*` vehicles stay on lane 1 in both runs (no
change needed once the blocker moved), so the one real lane change is the path under test.
