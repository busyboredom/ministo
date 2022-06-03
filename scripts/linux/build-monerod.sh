#!/bin/bash

# Set starting directory to script's directory
cd "$(dirname "${BASH_SOURCE[0]}")"

# Build Monerod
cd ../../monero
make release-static-linux-x86_64 -j$(nproc)

# Copy to external-bin/
cd ../
mkdir -p src-tauri/external-bin
cp monero/build/release/bin/monerod src-tauri/external-bin/monerod-x86_64-unknown-linux-gnu