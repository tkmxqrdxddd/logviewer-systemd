#!/bin/bash

# Run cargo install
cargo install --path .

# Move the binary to /usr/bin/ after installation
sudo mv /home/$USER/.cargo/bin/logviewer /usr/bin/

# Check the user's shell
if [ "$SHELL" == "/bin/bash/" ]; then
    # Restart bash shell
    exec bash
elif [ "$SHELL" == "/bin/zsh/" ]; then
    # Restart zsh shell
    exec zsh
else
    echo "Unsupported shell: $SHELL"
fi