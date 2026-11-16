# REPORT: city_milton_keynes (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **FAIL** | `SUMMARY: 15 differences in 15 groups (ref 12862 elements, cand 12858 elements)`. Our netconvert exit 0. |
| sim tripinfo (`compare_sim.py --tripinfo`) | **PASS** | MATCH (against the committed C++ reference net). |
| sim fcd (`compare_sim.py --fcd`) | **PASS** | MATCH. |
| sim stat (`compare_stat.py`) | **PASS** | `SUMMARY: 0 differences in 0 groups (ref 9 elements, cand 9 elements)`. |

The sim stage ran our sumo on the C++ net, so the sim verdict is independent of the net failure.

First net differences, in file order:
1. Junction `89966133` (a roundabout): `intLanes` ref has 5 internal lanes, ours has 4. Ours is missing
   `:89966133_4_0`, and edge `:89966133_4` is absent.
2. `junction 89966133 type`: ref `right_before_left`, ours `priority`.
3. Lane `:89966133_0_0` length 13.45 (ref) vs 13.06 (ours), and `:89966133_0_1` 13.45 vs 13.85.
4. Connection `1065536035->1553490413#0 0->1`: direction `l` (ref) vs `L` (ours), state `=` vs `m`.
5. Connection `1065536035->-1065536035 0->0` exists in the reference only.

The roundabout's internal lanes and connection set differ. Items 1 and 5 are structural. The sim MATCH should not
be read as proof of correctness at that junction: the routes may simply not use the missing connection (not checked).

Next step: reduce junction `89966133` to the smallest net that shows the missing internal lane and the
`priority` vs `right_before_left` type, then diff against C++ netconvert.
