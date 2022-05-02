use std::fs::{self, File};
use glob::glob;
use serde_json;
use serde::{Deserialize};
use std::io::{BufReader};
use std::env;
use std::collections::HashMap;

// #[allow(dead_code)]
// #[derive(Debug, Deserialize)]
// enum License {
//     Flat(String),
//     Nested {
//         r#type: String,
//         url: String
//     }
// }

#[derive(Debug, Deserialize)]
struct Package {
    license: Option<String>,
    version: Option<String>
}

// https://stackoverflow.com/a/58113997
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
    let mut search_dir = "./node_modules/";
    let mut licenses: HashMap<&String, u64> = HashMap::new();
    let mut count = 0;
    let no_license = "none".to_string();

    if args.len() > 1 {
        let arg = &args[1];
        match arg.as_str() {
            "--help" | "-h" => println!("Usage: {} [dir]", current_exe().unwrap()),
            _ => search_dir = arg
        }
    }

    for package_json_path in glob(&format!("{}/**/package.json", search_dir)).expect("Failed to read glob pattern") {
        if count == 100 {
            break;
        }

        count += 1;

        let filename = package_json_path.unwrap().display().to_string();

        if fs::symlink_metadata(filename.clone()).unwrap().is_symlink() {
            continue;
        }

        let file = File::open(filename.clone()).unwrap();
        let reader = BufReader::new(file);

        let package_json: Package = serde_json::from_reader(reader).unwrap();

        if package_json.version.is_some() {
            if let Some(license) = package_json.license {
                let val = licenses.entry(&license).or_insert(0);
                *val += 1;
            } else {
                *licenses.entry(&no_license).or_insert(0) += 1
            }
        }
    }

    println!("{:?}", licenses)
}
