#!/bin/bash

# Set starting directory to script's directory
cd "$(dirname "${BASH_SOURCE[0]}")"

# Triggered when the user interrupts the script to stop it
trap quitjobs INT
quitjobs() {
    echo ""
    pkill -P $$
    echo "Killed all running jobs".
    scriptCancelled="true"
    trap - INT
    exit
}

# Wait for user input so the jobs can be quit afterwards
scriptCancelled="false"
waitforcancel() {
    while :
    do
        if [ "$scriptCancelled" == "true" ]; then
            return
        fi
        sleep 1
    done
}

# Run ministo dev
cd ../../
devserver --noreload --path dist &
cargo tauri dev

# Trap the input and wait for the script to be cancelled
quitjobs
waitforcancel