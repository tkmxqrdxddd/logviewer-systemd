# Logviewer-systemd

Logviewer-systemd is a command-line tool that filters and saves system logs based on specified criteria.

## If it does not show anything, run it as root

## Usage


| Command | Description |
| --- | --- |
| `logviewer --help` | Display the help message. |
| `logviewer -s <path>` | Save logs to the specified path. |
| `logviewer -k <keyword>` | Filter logs by the specified keyword. |
| `logviewer -u <unit>` | Filter logs by the specified unit. |
| `logviewer -r` | Logging in real time |


## Dependencies

[Rust language](https://www.rust-lang.org/tools/install)

## Installation 
# From source:
```bash
$ git clone https://github.com/tkmxqrdxddd/logviewer-systemd
$ cd logviewer-systemd
$ cargo install --path -
```
# Arch Linux 
```bash
$ git clone https://github.com/tkmxqrdxddd/logviewer-systemd
$ cd logviewer-systemd
$ makepkg -si
```
# Debian
```bash
wget https://github.com/tkmxqrdxddd/logviewer-systemd/releases/download/v1.0.0/logviewer_1.0.0_amd64.deb
sudo dpkg -i https://github.com/tkmxqrdxddd/logviewer-systemd/releases/download/v1.0.0/logviewer_1.0.0_amd64.deb

