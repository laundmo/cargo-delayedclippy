use std::env;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let mut cmd = Command::new(env::var("CARGO").unwrap_or("cargo".into()));
    args.next(); // skip binary
    args.next(); // skip self
    let delay: u64 = env::var("DELAYEDCLIPPY_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(180); // default to 180ms, aka the "works on my machine" default
    sleep(Duration::from_millis(delay));
    cmd.arg("clippy").args(args).spawn()?;
    Ok(())
}
