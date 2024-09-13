#!/bin/bash

# Run cargo install
cargo install --path .

# Move the binary to /usr/bin/ after installation
sudo mv /home/$USER/.cargo/bin/logviewer /usr/bin/

