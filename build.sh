#!/bin/bash

# Build the Rust program
cargo build --release

# Check if the build was successful
if [ $? -eq 0 ]; then
    # Move the compiled binary to /usr/local/bin
    sudo mv target/release/logviewer /usr/local/bin/logviewer
    echo "Logviewer binary created at /usr/local/bin/logviewer"
else
    echo "Build failed. Check the error messages for more details."
fi

# Make the script executable
chmod +x build.sh
