use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut save_path = None;
    let mut keyword_filter = None;
    let mut unit_filter = None;
    let mut realtime_mode = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "--help" => {
                println!("Usage: logmaster [options]");
                println!("Options:");
                println!("  --help    Display this help message");
                println!("  -s <path> Save logs to the specified path");
                println!("  -k <keyword> Filter logs by keyword");
                println!("  -u <unit> Filter logs by unit");
                println!("  -r        Enable real-time logging mode");
                return;
            }
            "-s" => {
                if let Some(next_arg) = args.get(args.iter().position(|a| a == arg).map_or(0, |i| i + 1)) {
                    save_path = Some(next_arg.clone());
                } else {
                    println!("Error: Missing path for -s option");
                    return;
                }
            }
            "-k" => {
                if let Some(next_arg) = args.get(args.iter().position(|a| a == arg).map_or(0, |i| i + 1)) {
                    keyword_filter = Some(next_arg.clone());
                } else {
                    println!("Error: Missing keyword for -k option");
                    return;
                }
            }
            "-u" => {
                if let Some(next_arg) = args.get(args.iter().position(|a| a == arg).map_or(0, |i| i + 1)) {
                    unit_filter = Some(next_arg.clone());
                } else {
                    println!("Error: Missing unit for -u option");
                    return;
                }
            }
            "-r" => {
                realtime_mode = true;
            }
            _ => {}
        }
    }

    if realtime_mode {
        // Fetch and process logs in real-time
        loop {
            let output = Command::new("journalctl")
                .arg("-n")
                .arg("1000")
                .output()
                .expect("Failed to execute journalctl");

            let logs = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|s| s.to_string())
                .filter(|log| {
                    keyword_filter.as_ref().map_or(true, |k| log.contains(k))
                        && unit_filter.as_ref().map_or(true, |u| log.contains(u))
                })
                .collect::<Vec<String>>();

            if let Some(path) = &save_path {
                let mut file = File::create(path).expect("Failed to create save file");
                for log in logs {
                    file.write_all(format!("{}\n", log).as_bytes()).expect("Failed to write to save file");
                }
            } else {
                for log in logs {
                    println!("{}", log);
                }
            }

            // Sleep for a short duration before fetching logs again
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    } else {
        // Fetch and process logs once
        let output = Command::new("journalctl")
            .arg("-n")
            .arg("1000")
            .output()
            .expect("Failed to execute journalctl");

        let logs = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .filter(|log| {
                keyword_filter.as_ref().map_or(true, |k| log.contains(k))
                    && unit_filter.as_ref().map_or(true, |u| log.contains(u))
            })
            .collect::<Vec<String>>();

        if let Some(path) = &save_path {
            let mut file = File::create(path).expect("Failed to create save file");
            for log in logs {
                file.write_all(format!("{}\n", log).as_bytes()).expect("Failed to write to save file");
            }
        } else {
            for log in logs {
                println!("{}", log);
            }
        }
    }
}