# Sublanes — verdict (2026-11-16)

| Stage | Verdict | Detail |
|---|---|---|
| net | PASS | `compare_net.py` exact |
| sim (`ref`) | **FAIL — PANIC** | `crates/sumo_core/src/microsim/ms_leader_info.rs:503:46`: `index out of bounds: the len is 1 but the index is 1` |

C++ reference: 8 of 10 vehicles arrive by t=120. Our run aborts at the first sublane-leader lookup, before any output is written.

**Defect (not fixed here, case designer only):** `MSLeaderDistanceInfo::index_op` (`ms_leader_info.rs:503`)
indexes `myVehicles[sublane]` and `myDistances[sublane]` directly. The C++ `operator[]` asserts
`sublane < myVehicles.size()`. The panic says the vector has length 1 and index 1 was requested.
Not traced: which caller passes that sublane index, or whether `myVehicles` should have been sized larger.
The fixer should start from the callers of `index_op` during a sublane foe check.
