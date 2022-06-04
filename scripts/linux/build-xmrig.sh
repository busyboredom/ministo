#!/bin/bash

# Set starting directory to script's directory
cd "$(dirname "${BASH_SOURCE[0]}")"

# Build XMRig
mkdir -p ../../xmrig/build
cd ../../xmrig/scripts
./build_deps.sh
cd ../build
cmake .. -DXMRIG_DEPS=scripts/deps
make -j$(nproc)

# Copy to external-bin/
cd ../../
mkdir -p src-tauri/external-bin
cp xmrig/build/xmrig src-tauri/external-bin/xmrig-x86_64-unknown-linux-gnu