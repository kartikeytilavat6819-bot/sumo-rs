#!/bin/bash
# For agents, called from tools/kaggle/agent_run.sh on the VM gate only:
#   bash tools/gcloud/vm_cpp_trace.sh <ID> <patch-file> [target]   -> prints the path of an instrumented binary
# target = sumo (default) or netconvert. Copies the official 1.28.0 source tree (/home/gate/sumo-cpp, built by
# vm_cpp_setup.sh) to /home/gate/cpp-<ID> once with its OWN cmake build dir, resets its sources, applies your patch
# (a `git diff` against the official src/ tree, paths src/...), and rebuilds only the target (ccache keeps it fast).
# Never use it for references: those come only from the pristine wheel.
set -e
ID=$1 TARGET=${3:-sumo}
# patches written on Windows carry CRLF; git apply would reject them against the LF source tree
PATCH=$(mktemp); sed 's/\r$//' "$2" > "$PATCH"
D=/home/gate/cpp-$ID
[ -d /home/gate/sumo-cpp/bin ] || { echo "vm_cpp_trace: no C++ tree on this VM" >&2; exit 1; }
exec 8>/home/gate/cpp-$ID.lock; flock 8
# a copied build dir keeps the original's absolute paths and would rebuild /home/gate/sumo-cpp: configure our own
if [ ! -f "$D/build/CMakeCache.txt" ] || ! grep -q "CMAKE_HOME_DIRECTORY:INTERNAL=$D\$" "$D/build/CMakeCache.txt"; then
  rm -rf "$D"; mkdir -p "$D"
  (cd /home/gate/sumo-cpp && tar --exclude=./build --exclude=./bin -cf - .) | (cd "$D" && tar -xf -)
  mkdir -p "$D/build"
  (cd "$D/build" && cmake -DCMAKE_BUILD_TYPE=Release -DFOX_CONFIG= -DCHECK_OPTIONAL_LIBS=OFF -DENABLE_JAVA_BINDINGS=OFF \
     -DENABLE_CS_BINDINGS=OFF -DCMAKE_CXX_COMPILER_LAUNCHER=ccache .. >/dev/null)
fi
export CCACHE_BASEDIR=/home/gate CCACHE_NOHASHDIR=1
cd "$D"
git checkout -q -- src
git apply "$PATCH"
(cd build && make -j8 "$TARGET" 2>&1 | grep -E 'error|Error' | head -20) >&2
echo "$D/bin/$TARGET"
