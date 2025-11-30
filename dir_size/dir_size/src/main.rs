use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::fmt;

// momentan un config simplu doar cu un string pt calea dir
struct Config {
    dir: String,
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

    // rulam lgica din spate
    if let Err(e) = run(config) {
        eprintln!("Eroare la rulare: {}", e);
        std::process::exit(1);
    }
}

fn parse_args() -> Result<Config, MyError> {
    let mut args = env::args();

    // primul este numele programului(tbdone)
    let _program_name = args.next();

    // al doilea(calea)
    let dir_arg = match args.next() {
        Some(s) => s,
        None => return Err(MyError::NotEnoughArgs),
    };

    Ok(Config { dir: dir_arg })
}

//logica
fn run(config: Config) -> Result<(), MyError> {
    let path = Path::new(&config.dir);

    let total_size = calculate_dir_size(path)?;

    let nice = format_size(total_size);

    // formatul cerut
    println!("{} ({} bytes)", nice, total_size);

    Ok(())
}

fn calculate_dir_size(path: &Path) -> Result<u64, MyError> {
    let meta = fs::metadata(path)?;

    if meta.is_file() {
        return Ok(meta.len());
    }

    if meta.is_dir() {
        let mut sum: u64 = 0;
        let entries = fs::read_dir(path)?;

        for entry_res in entries {
            //tratam err cu match
            let entry = match entry_res {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Nu pot citi o intrare din director {:?}: {}", path, e);
                    continue;
                }
            };

            let child_path = entry.path();

            // recursiv
            let child_size_res = calculate_dir_size(&child_path);

            match child_size_res {
                Ok(sz) => {
                    // cu sat_add ca sa nu facem overflow
                    sum = sum.saturating_add(sz);
                }
                Err(MyError::IoError(e)) => {
                    //in caz de nu putem citi
                    eprintln!("Nu pot accesa {:?}: {}", child_path, e);
                }
                Err(MyError::NotEnoughArgs) => {
                    //
                    eprintln!("Eroare ciudata la {:?} (NotEnoughArgs)", child_path);
                }
            }
        }

        Ok(sum)
    } else {
        Ok(0)
    }
}

//fct pt bytes
fn format_size(bytes: u64) -> String {
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;

    let b = bytes as f64;

    if b >= gb {
        // un singur zecimal
        let value = b / gb;
        format!("{:.1}gb", value)
    } else if b >= mb {
        let value = b / mb;
        format!("{:.1}mb", value)
    } else if b >= kb {
        let value = b / kb;
        format!("{:.1}kb", value)
    } else {
        // foarte mic-> tot bytes
        format!("{}b", bytes)
    }
}

fn print_usage() {
    eprintln!("Utilizare:");
    eprintln!("  dir_size <cale_director>");
}
