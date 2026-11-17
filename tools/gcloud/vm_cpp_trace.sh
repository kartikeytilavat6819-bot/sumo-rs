#!/bin/bash
# For agents, called from tools/kaggle/agent_run.sh on the VM gate only:
#   bash tools/gcloud/vm_cpp_trace.sh <ID> <patch-file>   -> prints the path of an instrumented sumo binary
# Copies the prebuilt official 1.28.0 tree (/home/gate/sumo-cpp, built by vm_cpp_setup.sh) to /home/gate/cpp-<ID>
# once, resets its sources, applies your patch (a `git diff` made against the official src/ tree, paths src/...),
# and rebuilds only sumo (ccache keeps it fast). Never use it for references: those come only from the pristine wheel.
set -e
ID=$1
# patches written on Windows carry CRLF; git apply would reject them against the LF source tree
PATCH=$(mktemp); sed 's/\r$//' "$2" > "$PATCH"
D=/home/gate/cpp-$ID
[ -d /home/gate/sumo-cpp/bin ] || { echo "vm_cpp_trace: no C++ tree on this VM (Kaggle?)" >&2; exit 1; }
exec 8>/home/gate/cpp-$ID.lock; flock 8
[ -d "$D" ] || cp -a /home/gate/sumo-cpp "$D"
cd "$D"
git checkout -q -- src
git apply "$PATCH"
(cd build && make -j8 sumo 2>&1 | grep -E 'error|Error' | head -20) >&2
echo "$D/bin/sumo"
