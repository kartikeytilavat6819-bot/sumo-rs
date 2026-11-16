#!/bin/bash
# Temporary agent diagnostics (removed before merge).
set -u
OSM=tools/oracle/cases/city_munich/input.osm
RS=/tmp/target/debug/netconvert
CP=$SUMO_PRISTINE_BIN/netconvert
mkdir -p /tmp/nc
$CP --simple-projection true --osm-files $OSM -o /tmp/nc/cpp.net.xml > /tmp/nc/cpp.log 2>&1; echo "cpp exit=$?"
$RS --simple-projection true --osm-files $OSM -o /tmp/nc/rs.net.xml > /tmp/nc/rs.log 2>&1; echo "rs exit=$?"
python3 - <<'EOF'
import xml.etree.ElementTree as ET
J = "1413262637"
for tag, path in (("cpp", "/tmp/nc/cpp.net.xml"), ("rs", "/tmp/nc/rs.net.xml")):
    root = ET.parse(path).getroot()
    for j in root.iter("junction"):
        if j.get("id") == J:
            print(tag, "junction", {k: v for k, v in j.attrib.items() if k in ("type", "incLanes", "intLanes")})
    edges = {}
    for e in root.iter("edge"):
        if e.get("function") == "internal":
            continue
        edges[e.get("id")] = e
    for e in edges.values():
        if e.get("to") == J or e.get("from") == J:
            lanes = e.findall("lane")
            allow = lanes[0].get("allow") if lanes else None
            print(tag, "  ", "in " if e.get("to") == J else "out", e.get("id"), "type=" + str(e.get("type")), "from=" + str(e.get("from")), "lanes=" + str(len(lanes)), "allow=" + str(allow))
EOF
