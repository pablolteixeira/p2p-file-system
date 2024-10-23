#!/bin/bash

# This is a simple script to run a command in a new terminal multiple times

# Command to run
command="cargo run --bin main"

# Number of times to run the command
times=4

# Run the command in a new terminal multiple times
for ((i = 0; i <= times; i++)); do
    echo "Opening terminal for execution $i..."
    gnome-terminal -- bash -c "$command $i; echo 'Press Enter to close...'; read"
done
