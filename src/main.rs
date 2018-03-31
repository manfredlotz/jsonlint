#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args = clap_app!(fddf =>
                         (version: "0.9")
                         (author: "Manfred Lotz, 2018")
                         (about: "A JSON syntax checker and pretty printer.")
                         (@arg pretty_print: -p "Pretty prints a JSON file")
                         (@arg json_files: +required +multiple "JSON files to syntax check.")
    ).get_matches();



    let pretty_print = args.is_present("pretty_print");
    let files = args.values_of("json_files").unwrap();

    for element in files {

        let path = Path::new(&element);
        let display = path.display();

        match read_file(&path) {
            Err(why) => println!("{}: {}", display, why),
            Ok(s) =>
                if pretty_print {
                    match read_json(&s) {
                        Ok(v) => 
                            println!("{}", serde_json::to_string_pretty(&v).unwrap()),
                        Err(why) => println!("{}: {}", display, why)
                    }
                } else {
                    match check_json(&s) {
                        Some(e) => println!("{}: Not ok: {}", display, e),
                        _ => println!("{}: Ok", display)
                    }
                }
        }


    }
}

fn check_json(s: &str) -> Option<serde_json::Error>  { // serde_json::Value  {
    //let r : Result<serde_json::Value,serde_json::Error> = serde_json::from_str(s);
    let r : Result<serde::de::IgnoredAny,serde_json::Error> = serde_json::from_str(s);

    r.err()
}

fn read_json(s: &str) -> Result<serde_json::Value,serde_json::Error>  { // serde_json::Value  {
    serde_json::from_str(s)
}

fn read_file(path: &Path) -> Result<String, std::io::Error> {
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = File::open(path);

    match file {
        Err(why) => Err(why),
        Ok(mut file) =>
        {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => Err(why),
                Ok(_) => Ok(s)
            }
        }
    }

}
