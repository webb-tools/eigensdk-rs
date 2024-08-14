#!/bin/bash

# Define the directories
DIR1="./../avs/incredible-squaring-avs/contracts"
DIR2="./../avs/tangle-avs/contract"

# Function to run forge commands in a directory
run_forge_commands() {
    local dir=$1
    echo "Running forge commands in $dir"
    cd "$dir" || exit
    forge clean
    forge build
    cd - || exit
}

# Run forge commands in both directories
run_forge_commands "$DIR1"
run_forge_commands "$DIR2"

# Run cargo test in the starting directory
#echo "Running cargo test in the current directory"
#cargo test

echo "Rebuilt all AVS Smart Contracts!"