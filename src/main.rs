// Use gui-subsystem feature to hide console window for background tasks
// Build with: cargo build --release --features gui-subsystem
#![cfg_attr(
    all(not(debug_assertions), feature = "gui-subsystem"),
    windows_subsystem = "windows"
)]

use monitorconfig::cli;
use std::process;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
