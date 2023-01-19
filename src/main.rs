use std::{env::args, path::PathBuf, str::FromStr};

fn main() {
    let Some(arg) = args().skip(1).last() else {
        eprintln!("no file path provided");
        return;
    };

    let path = PathBuf::from_str(&arg).unwrap();

    if !path.exists() {
        eprintln!("path doesn't exist");
        return;
    }

    if !path.is_file() {
        eprintln!("path is not a regular file");
        return;
    }

    let file_content = std::fs::read_to_string(path).unwrap();
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(&file_content).unwrap();
    println!("Copied.");
}
