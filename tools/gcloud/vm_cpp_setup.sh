#!/bin/bash
# One-off (lead): build the official SUMO 1.28.0 C++ source on a gate VM at /home/gate/sumo-cpp,
# so agents can build instrumented copies there (vm_cpp_trace.sh) instead of in the laptop's WSL.
# Usage (as the ssh user): bash vm_cpp_setup.sh
set -e
if [ "$1" != build ]; then
  sudo apt-get update -q -o DPkg::Lock::Timeout=300 >/dev/null || sudo apt-get update -q
  sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -q -o DPkg::Lock::Timeout=300 \
    cmake libxerces-c-dev libproj-dev zlib1g-dev ccache --fix-missing >/dev/null
  cp "$0" /tmp/vm_cpp_build.sh && chmod 755 /tmp/vm_cpp_build.sh
  exec sudo -u gate -H bash /tmp/vm_cpp_build.sh build
fi
cd /home/gate
[ -d sumo-cpp ] || git clone -q --depth 1 -b v1_28_0 https://github.com/eclipse-sumo/sumo.git sumo-cpp
mkdir -p sumo-cpp/build && cd sumo-cpp/build
cmake -DCMAKE_BUILD_TYPE=Release -DFOX_CONFIG= -DCHECK_OPTIONAL_LIBS=OFF -DENABLE_JAVA_BINDINGS=OFF \
      -DENABLE_CS_BINDINGS=OFF -DCMAKE_CXX_COMPILER_LAUNCHER=ccache .. >/dev/null
make -j8 sumo netconvert 2>&1 | tail -n 2
../bin/sumo --version | head -n 1
