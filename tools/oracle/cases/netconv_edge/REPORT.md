# netconvert robustness: bad and unusual OSM input

**Status as of 2026-11-16** (re-run with the current binary, `run_all.sh` repointed at its own tree):

| # | Case | Now |
|---|------|-----|
| 1 | truncated OSM | **MATCH**: exit 1/1, same text and position (`unexpected end of input`, 10/1). |
| 2a | zero-byte file | **MATCH**: exit 1/1, same text and position (`invalid document structure`, 2/1). |
| 2c | XML error probes in `xml_probes/` | **MATCH** on all 12 probes: exit codes and full error text, including positions. |
| 2b, 3, 4a, 4b, 5a, 5b, 6 | | still MATCH |
| 0 | default projection | still DIFFERS (no PROJ binding), documented `shortcut:` |
| 7a/7b/7c | `real` variants | generated net now matches C++ line-for-line apart from the `generated on` stamp and the recorded input/output paths (verified on `base_ok`: 0 differing lines once those are excluded). The old "ours is 18 B larger" note was the version banner. |

Two things were fixed after this table was written:

1. **The panics are gone.** `sax_result` no longer panics; malformed XML exits 1 like C++.
2. **The version string was wrong**, which is why every generated net was a constant 19 bytes smaller:
   C++ writes `v1.28.0-0000000000` (from `version.h`), ours wrote `1.28.0`. Both binaries now use the
   exact reference string, so it matches in `--version`, in the net header and in every sim output.

3. **The malformed-XML messages now match C++** (`utils/xml/sumo_sax_reader.rs`). Our parser is quick-xml, so its
   errors are mapped onto Xerces wording and positions in the SAX layer, for the states measured on the gate VM:

| probe | C++ (= ours now) |
|---|---|
| truncated, cut inside an attribute value | `unexpected end of input`, at end of input |
| truncated after `lat=` / after a closed attribute | `attribute value expected` / `whitespace expected`, at end of input |
| clean EOF with no root (`<?xml ...?>` only, or empty file) | `invalid document structure`, at end of input |
| clean EOF inside an element | `input ended before all started tags were ended; last tag started is 'osm'`, at end of input |
| text before the root (`not xml at all`) | `invalid document structure`, at the first non-space character |
| text after the root, or a second root element | `comment or processing instruction expected`, at the start of the token |
| mismatched end tag `</osm>` inside `<node>` | `expected end of tag 'node'`, at the element name |
| undeclared entity `&bar;` in an attribute or in text | `entity 'bar' not found`, positioned after the `;` |

   Reproduce: `for f in xml_probes/*.osm; do netconvert --simple-projection true --osm-files $f -o /tmp/p.net.xml; done`
   with ours and with C++ (`SUMO_HOME` set), then compare the `Error:` lines and the exit codes.
   Not measured, so still quick-xml text: EOF inside a comment, PI, CDATA, DOCTYPE or in an unknown tag state
   (`shortcut:` in `read_error`). Unit test `utils/xml/tests.rs::errors` was updated to the C++ position for the mismatched tag (3/6, was 3/4).

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
