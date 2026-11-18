# Official-suite census (2026-11-18, wave2 e11e7a3e)

Every official SUMO 1.28.0 TextTest (sumo, netconvert, duarouter) was imported into a scratch copy on the gate VM
(tools/gcloud/census_run.sh), references generated with the official binaries only, and our port run against all of them.
Nothing of that scratch import is committed; these lists are the result.

| suite | official leaf tests | imported | PASS | FAIL | not comparable |
|---|---|---|---|---|---|
| sumo | 4707 | 4313 | 3695 | 292 (sumo_fail.txt) | 326 official sumo rejects the imported args (sumo_official_rejects.txt) + 394 not importable (297 expected-error, 41 no plain net, 34 no routes, 21 GUI/TraCI/tool, 1 no options) |
| netconvert | 1474 | 1474 | 1452 | 22 (netconvert_fail.tsv) | - |
| duarouter | 507 | 507 | 447 | 60 (duarouter_fail.tsv) | - |

Not in this census yet: TraCI suites (tests/traci, tests/complex: oc64/oc65 build their runners), polyconvert, od2trips,
jtrrouter, marouter, activitygen, dfrouter, netgenerate official suites.
