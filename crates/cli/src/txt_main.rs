use anyhow::Result;
use parser_core::DocumentParser;
use std::path::PathBuf;
use std::process::exit;
use txt_parser::TxtParser;

fn main() -> Result<()> {
    let path: PathBuf = std::env::args()
        .nth(1)
        .expect("Usage: txt-cli <fichier.txt>")
        .into();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    if ext != "txt" {
        eprintln!("Erreur : ce parser n'accepte que les fichiers .txt");
        exit(2); // code 2 : mauvaise extension
    }
    match TxtParser::parse(&path) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Erreur parsing TXT : {e}");
            exit(1); // code 1 : erreur parsing
        }
    }
}
