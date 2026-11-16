# netconvert robustness: bad and unusual OSM input

**Status as of 2026-11-16** (re-run with the current binary, `run_all.sh` repointed at its own tree):

| # | Case | Now |
|---|------|-----|
| 1 | truncated OSM | **MATCH on exit code** (1/1); no longer a panic. Message TEXT differs, see below. |
| 2a | zero-byte file | **MATCH on exit code** (1/1); no longer a panic. Message TEXT differs, see below. |
| 2b, 3, 4a, 4b, 5a, 5b, 6 | | still MATCH |
| 0 | default projection | still DIFFERS (no PROJ binding), documented `shortcut:` |
| 7a/7b/7c | `real` variants | generated net now matches C++ line-for-line apart from the `generated on` stamp and the recorded input/output paths (verified on `base_ok`: 0 differing lines once those are excluded). The old "ours is 18 B larger" note was the version banner. |

Two things were fixed after this table was written:

1. **The panics are gone.** `sax_result` no longer panics; malformed XML exits 1 like C++.
2. **The version string was wrong**, which is why every generated net was a constant 19 bytes smaller:
   C++ writes `v1.28.0-0000000000` (from `version.h`), ours wrote `1.28.0`. Both binaries now use the
   exact reference string, so it matches in `--version`, in the net header and in every sim output.

**Remaining, unfixed and real:** the malformed-XML *message text* differs, because our XML parser
reports its own diagnostics instead of SUMO's SAX messages:

| | C++ | ours |
|---|---|---|
| truncated | `Error: unexpected end of input` + `In file '...'` + `At line/column 10/1.` | `Error: syntax error: attribute value not closed: `"` not found before end of input` |
| zero-byte | `Error: invalid document structure` + `In file '...'` + `At line/column 2/1.` | `Error: Unexpected end of file` |

Same exit code and same refusal to continue; different words, and ours omits the file and position.
Fixing it means mapping parser errors onto SUMO's messages in the SAX layer — not string-patching the
call sites — so it stays a documented difference for now.

<details>
<summary>Original report (historical)</summary>


Binaries: C++ `netconvert` (WSL, `/home/karti/sumo-src/bin/netconvert`, `SUMO_HOME` set) vs Rust `netconvert.exe` (prebuilt, `C:\Users\karti\sumo-rs-target-main\debug`).
All runs used `--simple-projection true`, because the Rust port has no PROJ binding (see row 0).
Scripts: `run_all.sh` (cases 1-6), `run_real.sh` (case 7). Logs: `<case>/{cpp,rust}.log`.

| # | Case | C++ behaviour | Our behaviour | Verdict |
|---|------|---------------|---------------|---------|
| 0 | OSM import, default projection (no `--simple-projection`), `base_ok` | exit 0, network written | exit 1, `Error: Cannot import network data without PROJ-Library...` (`ni_frame.rs:812`, documented `shortcut:` at `ni_frame.rs:807`) | DIFFERS (known gap, not a panic) |
| 1 | Truncated OSM (cut mid-attribute) | exit 1, `Error: unexpected end of input`, line/col 10/1 | exit 1 but **PANIC**: `syntax error: attribute value not closed: `"` not found before end of input`, line/col 9/5 | FAIL: panic |
| 2a | Zero-byte file | exit 1, `Error: invalid document structure`, line/col 2/1 | exit 1 but **PANIC**: `Unexpected end of file`, line/col 2/1 | FAIL: panic |
| 2b | Valid empty `<osm/>` | exit 1, `Error: No nodes loaded.` | exit 1, `Error: No nodes loaded.` | MATCH |
| 3 | Way refs missing nodes (99, 98) | exit 0, warnings `geometry information (ref='99') is not known`, `Discarding way '11' because it has only 1 node(s)` | identical | MATCH |
| 4a | Duplicate node id | exit 0, no message | exit 0, no message | MATCH |
| 4b | Duplicate way id | exit 0, no message | exit 0, no message | MATCH |
| 5a | Unclosed `building=yes` area | exit 0, no message | exit 0, no message | MATCH |
| 5b | Way with one node | exit 0, `Discarding way '10' because it has only 1 node(s)` | identical | MATCH |
| 6 | `highway=construction`, `footway`, `oneway=reversible`, `lanes=0`, negative maxspeed | exit 0; `Skipping edge '10' because it has speed -8.33`, `New value for oneway found: reversible`, `Skipping edge '12' because it has zero lanes` | identical | MATCH |
| 7a | `real/input.osm`, default options | exit 0; net 189081 B | exit 0; net 189418 B; traffic-light warning aggregation text differs (`'%'` template) | exit MATCH, output DIFFERS |
| 7b | `real`, `--no-turnarounds true` | exit 0; net 182875 B | exit 0; net 183212 B | exit MATCH, output DIFFERS |
| 7c | `real`, `--junctions.corner-detail 3` | exit 0; net 185801 B | exit 0; net 186138 B | exit MATCH, output DIFFERS |

## Panics found

1. `thread 'main' panicked at crates\sumo_core\src\netimport\ni_importer_open_street_map.rs:60:26:` `syntax error: attribute value not closed: `"` not found before end of input` (truncated file). Raised in `sax_result` (line 59-61), called from the SAX section parsers at lines 2566, 2567, 2584 and 2799.
2. `thread 'main' panicked at crates\sumo_core\src\netimport\ni_importer_open_street_map.rs:60:26:` `Unexpected end of file` (zero-byte file). Same function, same call sites.

`sax_result` turns every SAX `ProcessError` into a panic. C++ reports these as a normal error and exits. Fix belongs in `sax_result` or its callers, which should return the error.

## Notes

- Row 0: on this machine the Rust binary cannot import any OSM without `--simple-projection`. C++ builds with PROJ. This is the known `shortcut:` in `ni_frame.rs`, not a robustness bug.
- Rows 1-2 are the same error class (XML parse error) with different text and positions. The panic is the real defect.
- Row 7: sizes and the traffic-light message aggregation differ. Only exit codes were compared; the nets were not diffed.
- Rust prints each traffic-light warning, then `'%'` for the summary line. C++ prints the summary with `Could not build program '%'`.

</details>
