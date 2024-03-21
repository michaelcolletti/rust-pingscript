use std::env;
use std::fs;
use std::process::Command;
use std::str::from_utf8;

const SEED: &str = "/etc/hosts";

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(6);
    if count < 1 || count > 50 {
        eprintln!("Count must be between 1 and 50");
        std::process::exit(1);
    }
    let seed = fs::read_to_string(SEED).expect("Failed to read seedfile.txt");
    for line in seed.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let _ip = parts.next().unwrap();
        if let Some(host) = parts.next() {
            println!("Pinging {} (count: {})", host, count);
            let output = Command::new("ping")
                .arg("-c")
                .arg(count.to_string())
                .arg(host)
                .output()
                .expect("Failed to execute ping command");
            println!("{}", from_utf8(&output.stdout).unwrap());
        }
    }
}
