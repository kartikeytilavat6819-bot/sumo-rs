# netconvert CLI option gap (Rust vs C++ 1.28.0)

- C++ options: 373 (from `netconvert --help`)
- Rust options: 331 (from `NIFrame`/`NBFrame`/`NWFrame` registrations)
- Missing: 42 (all are SystemFrame/GeoConvHelper/report/config options; no input/tls/building gaps by name)

Rust `--help` currently panics (`SubTopic 'Report' doesn't exist`), so counts come from source registration.

## Missing options by topic

| Topic | Missing options |
|---|---|
| Input | (none) |
| Output | human-readable-time, output.column-header, output.column-separator, output.compression, output.format, output-prefix, output-suffix, precision, precision.geo, write-license, write-metadata |
| Projection | proj, proj.dhdn, proj.dhdnutm, proj.inverse, proj.plain-geo, proj.rotate, proj.scale, proj.utm, simple-projection |
| Processing | xml-validation, xml-validation.net |
| TLS Building | (none) |
| Building | (none) |
| Config/Report/Logging | aggregate-warnings, configuration-file, error-log, help, language, log, log.processid, log.timestamps, message-log, no-warnings, print-options, random, save-commented, save-configuration, save-configuration.relative, save-schema, save-template, seed, verbose, version |

## Top 5 gaps most likely to change `.net.xml`

1. simple-projection / proj (coordinate projection)
2. proj.scale (coordinate scaling)
3. proj.rotate (coordinate rotation)
4. proj.utm (UTM projection selection)
5. seed / random (random tie-breaking in import/joins)

No code changes made.