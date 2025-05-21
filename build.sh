#!/bin/bash

# Build the application and generate a password with a length of 18 chars
cargo build --release && ./target/release/pigen 18

# Copy the executable to the projects root directory
cp ./target/release/pigen .

echo "To generate a new password, run: ./pigen"