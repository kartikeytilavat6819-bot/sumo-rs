#!/usr/bin/env bash
# oc122 probe (temporary): what PROJ / CPU does the official polyconvert use?
echo "arch: $(uname -m)"; grep -m1 "model name\|Model" /proc/cpuinfo; grep -m1 -o "fma" /proc/cpuinfo | head -1
echo "-- polyconvert version"; ${SUMO_PRISTINE_BIN}/polyconvert --version 2>&1 | head -3
echo "-- ldd"; ldd ${SUMO_PRISTINE_BIN}/polyconvert 2>&1 | grep -i "proj\|libm\.\|libc\." 
echo "-- libproj files"; ls -l /usr/lib/x86_64-linux-gnu/libproj* /usr/lib/aarch64-linux-gnu/libproj* /usr/local/lib/libproj* 2>/dev/null
echo "-- cs2cs"; cs2cs 2>&1 | head -1
echo "-- strings proj version in binary"; strings -a ${SUMO_PRISTINE_BIN}/polyconvert 2>/dev/null | grep -i -m5 "^[0-9]\.[0-9]\.[0-9]$\|Rel. [0-9]"
echo "-- gcc"; gcc --version 2>&1 | head -1
echo "-- glibc"; ldd --version 2>&1 | head -1
echo "AGENT RUN DONE"
