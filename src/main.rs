use std::{env};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use colored::*;
use structopt::StructOpt;
use walkdir::{WalkDir};
use pretty_bytes::converter::convert;

fn current_execution_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

/// Search for larger files in directories and display it in ranks.
#[derive(StructOpt)]
struct Cli {
    /// The root directory to read recursively
    #[structopt(parse(from_os_str))]
    directory: std::path::PathBuf,
}

struct Filex {
    size: u64,
    path: String
}

fn main() {
    let args = Cli::from_args();
    let directory = args.directory;
    let walker = WalkDir::new(directory).into_iter();
    let mut files: Vec<Filex> = Vec::new();
    for entry in walker {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let abs_path = entry.path().display().to_string();
            let f = File::open(&abs_path);
            let metadata = f.unwrap().metadata().unwrap();
            let file = Filex { path: abs_path.clone(), size: metadata.len() };
            files.push(file);
        }
    }

    files.sort_by(|a, b| b.size.cmp(&a.size));
    let max = if files.len() >= 10 {10} else {files.len()};
    for f in &files[0..max] {
        println!("{: >10} {}", convert(f.size as f64).green(), f.path);
    }
    println!();
}
