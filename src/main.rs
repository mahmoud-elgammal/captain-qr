#![allow(deprecated)]

use colored::Colorize;

use human_panic::setup_panic;

fn main() {
    setup_panic!();

    if let Err(e) = cqr::run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
