# Logviewer-systemd

Logviewer-systemd is a command-line tool that filters and saves system logs based on specified criteria.

## Usage


| Command | Description |
| --- | --- |
| `logviewer --help` | Display the help message. |
| `logviewer -s <path>` | Save logs to the specified path. |
| `logviewer -k <keyword>` | Filter logs by the specified keyword. |
| `logviewer -u <unit>` | Filter logs by the specified unit. |


## Dependencies

[Rust language](https://www.rust-lang.org/tools/install)

## Installation 
```bash
$ git clone https://github.com/tkmxqrdxddd/logviewer-systemd
$ cd logviewer-systemd
$ sh build.sh

