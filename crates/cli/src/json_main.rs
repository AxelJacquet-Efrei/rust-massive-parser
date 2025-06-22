use anyhow::Result;
use json_parser::JsonParser;
use std::path::PathBuf;
use std::process::exit;
use json_parser::convert_to_jsonl_mod::convert_to_jsonl;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _ = args.next(); // skip program name
    let path: PathBuf = args
        .next()
        .expect("Usage: json-cli <fichier.json> [mode|--to-jsonl]")
        .into();
    let mut mode = None;
    let mut to_jsonl = false;
    for arg in args {
        if arg == "--to-jsonl" {
            to_jsonl = true;
        } else {
            mode = Some(arg);
        }
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext != "json" && ext != "jsonl" {
        eprintln!("Erreur : ce parser n'accepte que les fichiers .json ou .jsonl");
        exit(2); // code 2 : mauvaise extension
    }
    if to_jsonl && ext == "json" {
        let jsonl_path = path.with_extension("jsonl");
        convert_to_jsonl(Path::new(&path), &jsonl_path)?;
        println!("Conversion terminée : {} -> {}", path.display(), jsonl_path.display());
        // On parse le JSONL généré
        match JsonParser::parse_mode(&jsonl_path, Some("jsonl")) {
            Ok(_) => exit(0),
            Err(e) => {
                eprintln!("Erreur parsing JSONL : {e}");
                exit(1);
            }
        }
    }
    if ext == "json" && JsonParser::detect_jsonl(&path).unwrap_or(false) == false {
        eprintln!("\x1b[33mAvertissement : Fichier JSON massif détecté. Pour des performances optimales, convertissez-le en JSONL (option --to-jsonl).\x1b[0m");
    }
    match JsonParser::parse_mode(&path, mode.as_deref()) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Erreur parsing JSON : {e}");
            exit(1);
        }
    }
}
