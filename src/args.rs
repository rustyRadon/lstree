use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to directory (default: current directory)
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Max depth to be displayed
    #[arg(short = 'd', long = "depth", default_value_t = usize::MAX)]
    pub depth: usize,

    /// Show only files, no directories
    #[arg(short, long)]
    pub only_files: bool,

    /// Show hidden files and directories
    #[arg(short = 'a', long = "all")]
    pub all: bool,
}
