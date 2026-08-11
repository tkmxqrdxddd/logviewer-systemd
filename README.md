# Logviewer-systemd

Logviewer-systemd is a command-line tool that filters and saves system logs based on specified criteria.

## If it does not show anything, run it as root

## Usage


| Command | Description |
| --- | --- |
| `logviewer --help` | Display the help message. |
| `logviewer -s <path>` | Save logs to the specified path. |
| `logviewer -k <keyword>` | Filter logs by the specified keyword. |
| `logviewer -u <unit>` | Show logs for the specified systemd unit. |
| `logviewer -n <lines>` | Number of lines to read from the journal (default: 1000). |
| `logviewer -b <boot>` | Show logs from a specific boot, e.g. `-b -1`. |
| `logviewer -r` | Follow new log entries in real time. |


## Dependencies

[Rust language](https://www.rust-lang.org/tools/install)

# Installation 
## From source:
```bash
$ git clone https://github.com/tkmxqrdxddd/logviewer-systemd
$ cd logviewer-systemd
$ cargo install --path .
```
## Arch Linux 

Use AUR Helper
```bash
$ paru -S logviewer-systemd
$ yay -S logviewer-systemd
```
Or install manually

```bash
$ git clone https://github.com/tkmxqrdxddd/logviewer-systemd
$ cd logviewer-systemd
$ makepkg -si
```
## Debian
```bash
wget https://github.com/tkmxqrdxddd/logviewer-systemd/releases/download/v1.0.0/logviewer_1.0.0_amd64.deb
sudo dpkg -i logviewer_1.0.0_amd64.deb

