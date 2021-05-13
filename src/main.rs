use std::env;
use std::ffi::OsStr;
use std::fs::{self};
use std::path::Path;

struct Params {
    exact: bool,
    extension: bool,
    term: String,
}

impl Params {
    pub fn new() -> Self {
        Params {
            exact: false,
            extension: false,
            term: "".to_string(),
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut params = Params::new();

    if args.len() < 2 {
        println!("Invalid arguments. A search term is required.");
        return;
    }

    for arg in args.iter() {
        match arg.as_str() {
            "-e" => {
                params.exact = true;
            }
            "-x" => {
                params.extension = true;
            }
            _ => {}
        }
    }
    params.term = args[args.len() - 1].to_string();
   // println!("{} {} {}", params.term, params.extension, params.exact);
    search(Path::new("./"), &params)

}

fn is_match(params: &Params, path: &Path) -> bool {
    if params.extension {
        if let Some(extension) = path.extension().and_then(OsStr::to_str) {
            return extension == params.term;
        }
    } else if let Some(filename) = path.file_name().and_then(OsStr::to_str) {
        if params.exact {
            return filename == params.term;
        } else {
            return filename.contains(&params.term);
        }
    }
    return false;
}

fn search(path: &Path, params: &Params) {
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = &entry.path();
            if path.is_dir() {
                search(&entry.path(), &params);
            } else {
                if is_match(params, path) {
                    println!("{}", entry.path().display());
                }
            }
        }
    }
}
