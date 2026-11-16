# Depart variants: Windows vs Linux

## Result

Linux (Kaggle, `main` at `ff57fad`) gave `PASS=13 FAIL=2`. Windows gave `PASS=15 FAIL=0`.
The two Linux failures were `lane_1` and `pos_max`. Both are error-parity cases.

## First difference

Not a numeric difference. The runner compares the first `Error:` line of ours and the reference
with `grep -m1`. The committed `ref/<variant>/stderr.txt` files use CRLF line endings, so the
reference line ends in `\r`. Our binary writes LF.

```
ref  (lane_1): 45 72 ... 27 76 33 27 2E 0D 0A  -> "...'v3'.\r"
ours (lane_1): 45 72 ... 27 76 33 27 2E 0A     -> "...'v3'."
```

## Root cause

`grep` differs by platform on CR handling:
- GNU grep (Linux) keeps `\r`, so the `=` test fails and the variant is marked FAIL.
- MSYS2 grep (Git Bash on Windows) strips `\r`, so the same test passes.

Verified in Git Bash: `grep -m1 "^Error:" ref/lane_1/stderr.txt` ends in `\n`, with no `\r`.

## Other checks

- `speed_0`: the Nov 15 `sumo-rs-target-main` binary gives `arrival=25.00`. The reference and the
  current build give `26.00`. This is a stale binary, not a platform effect. On Linux at `main`,
  `speed_0` PASSes.
- Hardcoded `/c/Users/...` output paths: on Linux they are created by `mkdir -p`, and the runner
  wrote outputs there. Not the cause.
- FCD: this runner does not write fcd output, and the references have no fcd. No FCD comparison was run.

## Fix

`tools/oracle/cases/depart/run_all_variants.sh`, line 26: strip CR from both sides before the comparison.

```sh
if [ $rc -ne 0 ] && [ "$(grep -m1 '^Error:' "$o/stderr.txt" | tr -d '\r')" = "$(grep -m1 '^Error:' "$ref/stderr.txt" | tr -d '\r')" ]; then
```

Shortcut noted in the script: upgrade to LF-only refs if the references are regenerated.
