#!/bin/bash
# TEMPORARY (nx03): like tools/gcloud/vm_cpp_trace.sh, but builds netconvert and replaces one source file.
#   bash tools/kaggle/vm_cpp_trace_nc.sh <ID> <file.cpp>   -> prints the path of an instrumented netconvert binary
set -e
ID=$1 FILE=$(realpath "$2")
D=/home/gate/cpp-$ID
[ -d /home/gate/sumo-cpp/bin ] || { echo "vm_cpp_trace_nc: no C++ tree on this VM" >&2; exit 1; }
exec 8>/home/gate/cpp-$ID.lock; flock 8
[ -d "$D" ] || cp -a /home/gate/sumo-cpp "$D"
cd "$D"
git checkout -q -- src
cp "$FILE" src/netbuild/NBNodeCont.cpp
touch src/netbuild/NBNodeCont.cpp
echo "cmake files pointing at pristine tree: $(grep -rl '/home/gate/sumo-cpp' "$D/build" --include='*.make' --include='link.txt' | wc -l)" >&2
grep -rl '/home/gate/sumo-cpp' "$D/build" --include='*.make' --include='link.txt' | xargs -r sed -i "s#/home/gate/sumo-cpp#$D#g"
find build -name 'NBNodeCont.cpp.o' -delete
(cd build && make -j8 netconvert > /tmp/cppnc-$ID.log 2>&1) || { grep -a -B3 -E 'rror|undefined' /tmp/cppnc-$ID.log | head -60 >&2; exit 1; }
grep -c JTRACE "$D/bin/netconvert" >&2 || { echo "vm_cpp_trace_nc: trace not compiled in" >&2; grep -a 'NBNodeCont' /tmp/cppnc-$ID.log | head -5 >&2; exit 1; }
echo "$D/bin/netconvert"
