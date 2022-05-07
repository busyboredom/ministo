#!/bin/bash

# Set starting directory to script's directory
cd "$(dirname "${BASH_SOURCE[0]}")"

# Build dependencies
./build-xmrig.sh
./build-p2pool.sh

# Build Ministo
cd ../../
cargo tauri build