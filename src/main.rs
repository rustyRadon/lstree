mod args;
mod scanner;
mod validator;

use clap::Parser;
use std::path::PathBuf;
use std::process;

fn main() {
    let args = args::Args::parse();

    // Clone so args can still be borrowed later
    let path: PathBuf = args.path.clone();

    if let Err(e) = validator::validate_path(&path) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    if let Err(e) = scanner::scan_directory(&path, &args) {
        eprintln!("Error scanning directory: {}", e);
        process::exit(1);
    }
}
