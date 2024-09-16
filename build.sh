#!/bin/bash

# Run cargo install
cargo install --path .

# Move the binary to /usr/bin/ after installation
sudo cp /home/$USER/.cargo/bin/logviewer /usr/bin/

