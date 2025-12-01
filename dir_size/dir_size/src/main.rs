use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

// config acum are si filters
struct Config {
    dir: String,
    filters: Vec<String>,
}

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
            MyError::NotEnoughArgs => write!(f, "nu ai dat destule argumente (lipsește calea)"),
            MyError::IoError(e) => write!(f, "eroare de I/O: {}", e),
        }
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

    if let Err(e) = run(config) {
        eprintln!("Eroare la rulare: {}", e);
        std::process::exit(1);
    }
}

// acum citim si --filter <pattern>
fn parse_args() -> Result<Config, MyError> {
    let mut args = env::args();

    // primul e numele programului
    let _program_name = args.next();

    let mut dir_arg: Option<String> = None;
    let mut filters: Vec<String> = Vec::new();

    // parcurgem restul argumentelor
    while let Some(arg) = args.next() {
        if arg == "--filter" {
            if let Some(pat) = args.next() {
                filters.push(pat);
            } else {
                eprintln!("Avertisment: --filter fara pattern, il ignor");
            }
        } else {
            if dir_arg.is_none() {
                dir_arg = Some(arg);
            } else {
                eprintln!("Avertisment: argument necunoscut sau in plus: {}", arg);
            }
        }
    }

    let dir = match dir_arg {
        Some(d) => d,
        None => return Err(MyError::NotEnoughArgs),
    };

    Ok(Config { dir, filters })
}

fn run(config: Config) -> Result<(), MyError> {
    let path = Path::new(&config.dir);

    let total_size = calculate_dir_size(path, &config.filters)?;

    let nice = format_size(total_size);

    println!("{} ({} bytes)", nice, total_size);

    Ok(())
}

// am adaugat filtre
fn calculate_dir_size(path: &Path, filters: &Vec<String>) -> Result<u64, MyError> {
    let meta = fs::metadata(path)?;

    if meta.is_file() {
        // daca e fisier, vedem daca trece de filtre
        // daca nu sunt filtre, il numaram oricum
        let file_name_opt = path.file_name().and_then(|n| n.to_str());

        if let Some(name) = file_name_opt {
            if file_matches_filters(name, filters) {
                return Ok(meta.len());
            } else {
                // nu se potriveste cu niciun filtru
                return Ok(0);
            }
        } else {
            // nume ciudat nu putem lua &str il sarim
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
        Ok(0)
    }
}

// functie simpla: momentan filtrele sunt doar "substring"
fn file_matches_filters(name: &str, filters: &Vec<String>) -> bool {
    if filters.is_empty() {
        // daca nu e niciun filtru rice fisier e ok
        return true;
    }

    for f in filters {
        if name.contains(f) {
            return true;
        }
    }

    false
}

// formatam marimea in kb/mb/gb
fn format_size(bytes: u64) -> String {
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

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  dir_size <dir> [--filter pattern] [--filter pattern2] ...");
}
