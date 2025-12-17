use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use regex::Regex;

// config
struct Config {
    root: PathBuf,
    filters: Vec<String>,
}

// err
enum MyError {
    NotEnoughArgs,
    IoError(io::Error),
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> MyError {
        MyError::IoError(e)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotEnoughArgs => write!(f, "nu ai dat destule argumente (lipseste calea)"),
            MyError::IoError(e) => write!(f, "eroare de I/O: {}", e),
        }
    }
}

trait SizeFormatter {
    fn format_size(&self, bytes: u64) -> String;
}

struct HumanSizeFormatter;

impl SizeFormatter for HumanSizeFormatter {
    fn format_size(&self, bytes: u64) -> String {
        let kb = 1024.0;
        let mb = kb * 1024.0;
        let gb = mb * 1024.0;

        let b = bytes as f64;

        if b >= gb {
            let value = b / gb;
            format!("{:.1}gb", value)
        } else if b >= mb {
            let value = b / mb;
            format!("{:.1}mb", value)
        } else if b >= kb {
            let value = b / kb;
            format!("{:.1}kb", value)
        } else {
            format!("{}b", bytes)
        }
    }
}

// structura
struct DirSizeApp {
    config: Config,
    filters: Vec<Regex>,
    formatter: HumanSizeFormatter,
}

impl DirSizeApp {
    fn new(config: Config) -> DirSizeApp {
        let filters = build_regex_filters(&config.filters);
        DirSizeApp {
            config,
            filters,
            formatter: HumanSizeFormatter,
        }
    }

    fn run(&self) -> Result<(), MyError> {
        let root_path = &self.config.root;

        let total_size = calculate_dir_size(root_path, &self.filters)?;

        let pretty = self.formatter.format_size(total_size);

        println!("{} ({} bytes)", pretty, total_size);

        Ok(())
    }
}

fn main() {
    let config = match parse_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Eroare la argumente: {}", e);
            print_usage();
            std::process::exit(1);
        }
    };

    let app = DirSizeApp::new(config);

    if let Err(e) = app.run() {
        eprintln!("Eroare la rulare: {}", e);
        std::process::exit(1);
    }
}

fn parse_args() -> Result<Config, MyError> {
    let mut args = env::args();

    // primul e numele programului (nu-l folosim)
    args.next();

    let mut root: Option<PathBuf> = None;
    let mut filters: Vec<String> = Vec::new();

    // parcurgem toate argumentele
    while let Some(arg) = args.next() {
        if arg == "--filter" {
            if let Some(pat) = args.next() {
                filters.push(pat);
            } else {
                eprintln!("Avertisment: --filter fara pattern, il ignor");
            }
        } else if arg == "--help" {
            print_usage();
            std::process::exit(0);
        } else if root.is_none() {
            root = Some(PathBuf::from(arg));
        } else {
            eprintln!("Avertisment: argument necunoscut sau in plus: {}", arg);
        }
    }

    let root_path = match root {
        Some(p) => p,
        None => return Err(MyError::NotEnoughArgs),
    };

    Ok(Config {
        root: root_path,
        filters,
    })
}

// transformam string in regex
fn build_regex_filters(patterns: &[String]) -> Vec<Regex> {
    let mut result = Vec::new();

    for pat in patterns {
        match Regex::new(pat) {
            Ok(r) => result.push(r),
            Err(e) => {
                eprintln!("Avertisment: nu pot compila regex-ul \"{}\": {}", pat, e);
            }
        }
    }

    result
}

// functie recursiva ce calculeaza dimensiunea
fn calculate_dir_size(path: &Path, filters: &[Regex]) -> Result<u64, MyError> {
    let meta = fs::metadata(path)?;

    if meta.is_file() {
        let file_name_opt = path.file_name().and_then(|n| n.to_str());

        if let Some(name) = file_name_opt {
            if file_matches_filters(name, filters) {
                return Ok(meta.len());
            } else {
                return Ok(0);
            }
        } else {
            // nume "ciudat
            return Ok(0);
        }
    }

    if meta.is_dir() {
        let mut sum: u64 = 0;
        let entries = fs::read_dir(path)?;

        for entry_res in entries {
            let entry = match entry_res {
                Ok(e) => e,
                Err(e) => { 
                    eprintln!("Nu pot citi o intrare din director {:?}: {}", path, e);
                    continue;
                }
            };

            let child_path = entry.path();

            let child_size_res = calculate_dir_size(&child_path, filters);

            match child_size_res {
                Ok(sz) => {
                    sum = sum.saturating_add(sz);
                }
                Err(MyError::IoError(e)) => {
                    eprintln!("Nu pot accesa {:?}: {}", child_path, e);
                }
                Err(MyError::NotEnoughArgs) => {
                    eprintln!("Eroare ciudata la {:?} (NotEnoughArgs)", child_path);
                }
            }
        }

        Ok(sum)
    } else {
        // pentru alte tipuri
        Ok(0)
    }
}

fn file_matches_filters(name: &str, filters: &[Regex]) -> bool {
    if filters.is_empty() {
        return true;
    }

    for r in filters {
        if r.is_match(name) {
            return true;
        }
    }

    false
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  dir_size <dir> [--filter \"regex1\"] [--filter \"regex2\"] ...");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  dir_size ./some_folder");
    eprintln!("  dir_size ./some_folder --filter \".*\\.exe\" --filter \".*\\.dll\"");
}
