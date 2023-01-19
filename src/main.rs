use std::{env::args, path::PathBuf, str::FromStr};

fn main() {
    let arg: Vec<String> = args().skip(1).take(1).collect();

    if arg.is_empty() {
        panic!("no file path provided");
    }

    let path = PathBuf::from_str(&arg[0]).unwrap();

    if !path.exists() {
        panic!("path doesn't exist");
    }

    if !path.is_file() {
        panic!("path is not a regular file");
    }

    let file_content = std::fs::read_to_string(path).unwrap();
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(&file_content).unwrap();
    println!("Copied.");
}
