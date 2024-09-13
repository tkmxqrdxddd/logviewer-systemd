# Logviewer-systemd

Logviewer-systemd is a command-line tool that filters and saves system logs based on specified criteria.

## Usage


| Command | Description |
| --- | --- |
| `logmviewer --help` | Display the help message. |
| `logmviewer -s <path>` | Save logs to the specified path. |
| `logmviewer -k <keyword>` | Filter logs by the specified keyword. |
| `logmviewer -u <unit>` | Filter logs by the specified unit. |

## Installation 
```bash
$ cd logviewer-systemd
$ sudo ./build.sh

