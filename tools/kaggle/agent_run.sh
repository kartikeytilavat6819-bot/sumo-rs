#!/bin/bash
# TEMPORARY agent run (residual state body diff). Removed before the final commit.
set -u
OUT=/tmp/rt
for c in city_manhattan_midtown city_plano; do
  echo "== $c residual diff (excluding route lines)"
  diff <(grep -v '<route id=' $OUT/$c/cpp/s.xml) <(grep -v '<route id=' $OUT/$c/ours/s.xml) | head -40 | cut -c1-260
  echo "== $c diff count by tag (ours vs cpp)"
  diff <(grep -o '<[a-zA-Z_-]*' $OUT/$c/cpp/s.xml | sort | uniq -c) <(grep -o '<[a-zA-Z_-]*' $OUT/$c/ours/s.xml | sort | uniq -c)
done
echo "AGENT RUN DONE"
