use std::fs::File;
use glob::glob;
use serde_json;
use serde::{Deserialize};
use std::io::{BufReader};
use std::env;
use std::default::Default;

#[derive(Debug, Deserialize)]
struct LicenseExtended {
    r#type: String,
}

#[derive(Debug, Deserialize)]
enum License {
    String,
    LicenseExtended,
}

#[derive(Deserialize)]
struct Package {
    license: Option<License>,
}

#[derive(Deserialize, Debug)]
struct MyType(Vec<i32>);
impl Default for MyType {
    fn default() -> Self {
        Self(vec![1, 2, 3])
    }
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
            "--help" => println!("Usage: {} [dir]", current_exe().unwrap()),
            _ => {dir = arg}
        }
    }

    for entry in glob(&format!("{}/**/package.json", dir)).expect("Failed to read glob pattern") {
        let filename = entry.unwrap().display().to_string();
        let file = File::open(filename.clone()).unwrap();
        let reader = BufReader::new(file);

        let json: Package = serde_json::from_reader(reader).unwrap();
        if let Some(license) = json.license {
            println!("{}: {:?}", filename, license);
        } else {
            println!("{}: none", filename);
        }
    }
}
