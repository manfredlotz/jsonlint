use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::Parser;
// use clap::{Command, CommandFactory, Parser, ValueHint};
// use clap_complete::{generate, Generator, Shell};

#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
#[clap(about = "A JSON syntax checker and pretty printer.")]
struct Args {
    #[clap(short = 'p', help = "Pretty prints a JSON file")]
    pretty_print: bool,
    #[clap(help = "JSON files to syntax check")]
    json_files: Vec<String>,
}

fn main() {
    let Args {
        pretty_print,
        json_files,
    } = Args::parse();

    for element in json_files {
        let path = Path::new(&element);
        let display = path.display();

        match read_file(path) {
            Err(why) => println!("{}: {}", display, why),
            Ok(s) => {
                if pretty_print {
                    match read_json(&s) {
                        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
                        Err(why) => println!("{}: {}", display, why),
                    }
                } else {
                    match check_json(&s) {
                        Some(e) => println!("{}: Not ok: {}", display, e),
                        _ => println!("{}: Ok", display),
                    }
                }
            }
        }
    }
}

fn check_json(s: &str) -> Option<serde_json::Error> {
    //let r : Result<serde_json::Value,serde_json::Error> = serde_json::from_str(s);
    let r: Result<serde::de::IgnoredAny, serde_json::Error> = serde_json::from_str(s);

    r.err()
}

fn read_json(s: &str) -> Result<serde_json::Value, serde_json::Error> {
    // serde_json::Value  {
    serde_json::from_str(s)
}

fn read_file(path: &Path) -> Result<String, std::io::Error> {
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(path)?;

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => Err(why),
        Ok(_) => Ok(s),
    }
}
