#!/bin/bash
# Temporary agent audit: option templates, ours vs pristine (removed before merge).
set -u
T=/tmp/tpl
mkdir -p $T
RS_DIR=/tmp/target/debug
CP_DIR=$SUMO_PRISTINE_BIN
$CP_DIR/netconvert --save-template $T/cpp_nc.xml > $T/cpp_nc.log 2>&1; echo "cpp netconvert template exit=$?"
$RS_DIR/netconvert --save-template $T/rs_nc.xml > $T/rs_nc.log 2>&1; echo "rs netconvert template exit=$?"
$CP_DIR/sumo --save-template $T/cpp_sumo.xml > $T/cpp_sumo.log 2>&1; echo "cpp sumo template exit=$?"
$RS_DIR/sumo --save-template $T/rs_sumo.xml > $T/rs_sumo.log 2>&1; echo "rs sumo template exit=$?"
python3 - <<'EOF'
import xml.etree.ElementTree as ET
def leaves(path):
    root = ET.parse(path).getroot()
    out = {}
    def walk(e, p):
        kids = list(e)
        if not kids:
            out["/".join(p)] = dict(e.attrib)
        for k in kids:
            walk(k, p + [k.tag])
    for k in root:
        walk(k, [k.tag])
    return out
for prog in ("nc", "sumo"):
    a = leaves(f"/tmp/tpl/cpp_{prog}.xml")
    b = leaves(f"/tmp/tpl/rs_{prog}.xml")
    print(f"== {prog}: cpp leaves={len(a)} rs leaves={len(b)}")
    for k in sorted(set(a) - set(b)):
        print(f"ONLY_CPP {k} {a[k]}")
    for k in sorted(set(b) - set(a)):
        print(f"ONLY_RS {k} {b[k]}")
    for k in sorted(set(a) & set(b)):
        if a[k] != b[k]:
            print(f"DIFF {k} cpp={a[k]} rs={b[k]}")
EOF
