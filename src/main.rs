use std::{
    env::args,
    fs::OpenOptions,
    io::{BufReader, Read},
    path::PathBuf,
};

use flate2::read::GzDecoder;

fn main() {
    let Some(path) = args().skip(1).last().map(PathBuf::from) else {
        eprintln!("no file path provided");
        return;
    };

    if !path.exists() {
        eprintln!("path doesn't exist");
        return;
    }

    if !path.is_file() {
        eprintln!("path is not a regular file");
        return;
    }

    let file_content = if let Some("gz") = path.extension().and_then(|s| s.to_str()) {
        let file = OpenOptions::new().read(true).open(&path).unwrap();

        let mut d = GzDecoder::new(BufReader::new(file));
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();
        s
    } else {
        std::fs::read_to_string(&path).unwrap()
    };

    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(&file_content).unwrap();
    println!("Copied.");
}
