use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]

pub struct Args {
    ///path to dir (current folder = default)
    pub path: Option<String>,

    ///max depth to be displayed 
    #[arg(short = "d", long = "depth", default_value = usize::max)]
    pub depth: usize,

    ///show only files. no dir
    #[arg(short, long)]
    pub only_file: bool,
}