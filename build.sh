#!/bin/bash

# Build the Rust program
cargo build --release

# Check if the build was successful
if [ $? -eq 0 ]; then
    # Move the compiled binary to /usr/local/bin
    sudo mv target/release/logmaster /usr/local/bin/logmaster
    echo "Logmaster binary created at /usr/local/bin/logmaster"
else
    echo "Build failed. Check the error messages for more details."
fi

# Make the script executable
chmod +x build.sh