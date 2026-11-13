# Oracle cases: netconvert status

Run: `cargo run -q -p netconvert -- --osm-files tools/oracle/cases/<case>/input.osm --simple-projection -o C:\Users\karti\Temp\rb_<case>.net.xml`
Compare: `python tools/oracle/compare_net.py tools/oracle/cases/<case>/ref.net.xml C:\Users\karti\Temp\rb_<case>.net.xml`

| case | netconvert | elements (ref / cand) | groups | categories | blocker |
|---|---|---|---|---|---|
| junction | Success (from lead's run) | 216 / not re-run | not re-run | lane shape only (per lead) | none |
| roundabout | Success | 76 / 76 | 60 | shape 31, length 13, speed 4, x 6, y 6 (junction positions) | none for netconvert. Likely cause: `--simple-projection` here vs the reference's UTM (`+proj=utm +zone=33`); convBoundary is 334.89 wide vs 294.47. Not yet confirmed by a UTM run. No connection or tlLogic differences. |
| signal | Panics: `NBTrafficLightDefinition::collectLinks is pure virtual` (nb_traffic_light_definition.rs:997) | 75 (ref) / no output | n/a | n/a | `NBTrafficLightDefinition::collectLinks` vtable not implemented (owned by tl01). |

## Notes

- Roundabout ran to `Success` with no panic, so no porting was needed.
- Roundabout emits three `Could not find corresponding edge or compatible lane for free-floating pt stop` warnings (Fliederweg, Dorfaue). Not compared against C++ yet.
- Signal ref uses UTM zone 28; a comparison also needs a UTM run, not `--simple-projection`.
