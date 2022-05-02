use std::fs::{self, File};
use glob::glob;
use serde_json;
use serde::{Deserialize};
use std::io::{BufReader};
use std::env;

#[derive(Debug, Deserialize)]
enum License {
    String(String),
    Nested {
        r#type: Option<String>,
        url: Option<String>
    }
}

#[derive(Debug, Deserialize)]
struct Package {
    license: Option<License>,
    version: Option<String>
}

fn current_exe() -> Option<String> {
    env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dir = "./node_modules/";

    if args.len() > 1 {
        let arg = &args[1];
        match arg.as_str() {
            "--help" | "-h" => println!("Usage: {} [dir]", current_exe().unwrap()),
            _ => dir = arg
        }
    }

    for entry in glob(&format!("{}/**/package.json", dir)).expect("Failed to read glob pattern") {
        let filename = entry.unwrap().display().to_string();

        let metadata = fs::symlink_metadata(filename.clone()).unwrap();

        if metadata.is_symlink() {
            continue;
        }

        let file = File::open(filename.clone()).unwrap();
        let reader = BufReader::new(file);

        let package: Package = serde_json::from_reader(reader).unwrap();
        if package.version.is_some() {
            match package.license {
                Some(license) => println!("{}: {:?}", filename, license),
                _ => println!("{}: none", filename)
            }
        }
    }
}
