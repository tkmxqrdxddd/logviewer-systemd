use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "A simple log viewer and filter for journalctl")]
struct Args {
    /// Save logs to the specified path
    #[arg(short, long)]
    save: Option<String>,

    /// Filter logs by keyword
    #[arg(short, long)]
    keyword: Option<String>,

    /// Show logs for a systemd unit (forwarded to journalctl -u)
    #[arg(short, long)]
    unit: Option<String>,

    /// Number of lines to read from the journal
    #[arg(short = 'n', long, default_value_t = 1000)]
    lines: usize,

    /// Start at a specific boot (forwarded to journalctl -b), e.g. "-1"
    #[arg(short, long)]
    boot: Option<String>,

    /// Follow new log entries as they are written
    #[arg(short = 'r', long)]
    follow: bool,
}

fn main() {
    let args = Args::parse();

    let mut command = Command::new("journalctl");
    command.arg("-n").arg(args.lines.to_string());
    if let Some(unit) = &args.unit {
        command.arg("-u").arg(unit);
    }
    if let Some(boot) = &args.boot {
        command.arg("-b").arg(boot);
    }
    if args.follow {
        command.arg("-f");
    }

    let mut child = match command
        .stdout(Stdio::piped())
        // journalctl is a pager-aware program; avoid it detecting a tty
        .env("PAGER", "cat")
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            eprintln!("Failed to execute journalctl: {err}");
            std::process::exit(1);
        }
    };

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            eprintln!("Failed to capture journalctl output");
            std::process::exit(1);
        }
    };

    let mut file = args.save.as_ref().map(|path| open_save_file(path));

    for line in BufReader::new(stdout).lines() {
        match line {
            Ok(line) => {
                if filter(&line, &args) {
                    output(&line, &mut file);
                }
            }
            Err(err) => {
                eprintln!("Failed to read journalctl output: {err}");
                break;
            }
        }
    }

    match child.wait() {
        Ok(status) if status.success() => {}
        Ok(status) => eprintln!("journalctl exited with status {status}"),
        Err(err) => eprintln!("Failed to wait for journalctl: {err}"),
    }
}

fn filter(line: &str, args: &Args) -> bool {
    args.keyword.as_ref().is_none_or(|k| line.contains(k))
}

fn output(line: &str, file: &mut Option<File>) {
    match file {
        Some(file) => {
            if let Err(err) = writeln!(file, "{line}") {
                eprintln!("Failed to write to save file: {err}");
            }
        }
        None => println!("{line}"),
    }
}

fn open_save_file(path: &str) -> File {
    match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create save file '{path}': {err}");
            std::process::exit(1);
        }
    }
}
