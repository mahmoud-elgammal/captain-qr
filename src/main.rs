use colored::Colorize;

fn main() {
    if let Err(e) = cqr::run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
