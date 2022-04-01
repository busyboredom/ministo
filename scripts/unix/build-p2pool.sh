#!/bin/bash

# Set starting directory to script's directory
cd "$(dirname "${BASH_SOURCE[0]}")"

# Build P2Pool
mkdir -p ../../p2pool/build
cd ../../p2pool/build
cmake ..
make -j$(nproc)

# Copy to external-bin/
cd ../../
mkdir -p src-tauri/external-bin
cp p2pool/build/p2pool src-tauri/external-bin/p2pool-x86_64-unknown-linux-gnu