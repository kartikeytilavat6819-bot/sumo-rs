#!/bin/bash
# Gate helper for oc68: locate netopts_new output wherever the gate put it and print panic details.
set -u
S=""
for root in /tmp /var/tmp /workspace "$HOME" /root /home /mnt /data .; do
  [ -d "$root" ] || continue
  f=$(find "$root" -maxdepth 7 -name summary.tsv 2>/dev/null | head -1)
  if [ -n "$f" ]; then S="$f"; break; fi
done
echo "SUMMARY=$S"
if [ -n "$S" ]; then
  echo "=== NON-PASS ROWS ==="
  grep -v $'\tPASS\t' "$S" | head -60
  echo "=== COUNTS ==="
  cut -f2 "$S" | sort | uniq -c
fi
echo "=== PANIC LOGS ==="
for row in itt_nc_netconvert_function_renaming_joinedTLS_renumber itt_nc_netconvert_function_renaming_prefix_joinedTLS; do
  for root in /tmp /var/tmp /workspace "$HOME" /root /home .; do
    [ -d "$root" ] || continue
    f=$(find "$root" -maxdepth 9 -type f -path "*/$row/rust.log" 2>/dev/null | head -1)
    if [ -n "$f" ]; then echo "--- $row : $f"; grep -m1 -A6 panicked "$f"; fi
  done
done
echo "=== DONE ==="
