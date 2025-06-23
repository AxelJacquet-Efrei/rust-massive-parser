use anyhow::Result;
use json_parser::JsonParser;
use std::path::PathBuf;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _ = args.next(); // skip program name
    let path: PathBuf = args
        .next()
        .expect("Usage: json-cli <fichier.json/jsonl>")
        .into();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext != "json" && ext != "jsonl" {
        eprintln!("Erreur : ce parser n'accepte que les fichiers .json ou .jsonl");
        exit(2); // code 2 : mauvaise extension
    }
    match JsonParser::parse_auto(&path) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Erreur parsing JSON : {e}");
            exit(1); // code 1 : erreur parsing
        }
    }
}
