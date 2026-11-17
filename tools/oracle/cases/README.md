# Oracle cases

Each subdirectory contains `input.osm`, a C++ reference `ref.net.xml` (when <= 200 kB),
`routes.rou`, and `CASE.md`.

## Regenerate a reference net on any machine

Requires SUMO (netconvert) installed and `SUMO_HOME` set to the SUMO root.

```
SUMO_HOME=/path/to/sumo netconvert \
  --osm-files tools/oracle/cases/<name>/input.osm \
  -o tools/oracle/cases/<name>/ref.net.xml
```

On Windows via WSL (as used to build these refs):

```
wsl -e bash -lc "SUMO_HOME=\$HOME/sumo-src \$HOME/sumo-src/bin/netconvert --osm-files /mnt/c/Users/<user>/sumo-rs/tools/oracle/cases/<name>/input.osm -o /mnt/c/Users/<user>/sumo-rs/tools/oracle/cases/<name>/ref.net.xml"
```

Regenerate all three with:

```
for c in junction roundabout signal; do
  SUMO_HOME=$SUMO_HOME netconvert --osm-files tools/oracle/cases/$c/input.osm -o tools/oracle/cases/$c/ref.net.xml
done
```

If a regenerated `ref.net.xml` exceeds 200 kB, remove it from the commit and note the size in that case's `CASE.md`.