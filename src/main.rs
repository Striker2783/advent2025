use std::{env::args, path::PathBuf, process::exit};

use advent2025::run;

fn main() {
    let mut args = args();
    args.next();
    let day = match args.next() {
        Some(a) => match a.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                exit(0);
            }
        },
        None => {
            eprintln!("No day inputted");
            exit(0);
        }
    };
    let file = match args.next() {
        Some(f) => PathBuf::from(f),
        None => {
            eprintln!("No file provided");
            exit(0)
        }
    };
    run(day, &file);
}
