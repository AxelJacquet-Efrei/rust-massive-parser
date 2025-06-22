use anyhow::Result;
use std::path::PathBuf;
use std::process::exit;
use csv_parser::CsvParser;
use parser_core::DocumentParser;

fn main() -> Result<()> {
    let path: PathBuf = std::env::args()
        .nth(1)
        .expect("Usage: csv-cli <fichier.csv>")
        .into();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    if ext != "csv" && ext != "tsv" {
        eprintln!("Erreur : ce parser n'accepte que les fichiers .csv ou .tsv");
        exit(2); // code 2 : mauvaise extension
    }
    match CsvParser::parse(&path) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Erreur parsing CSV : {e}");
            exit(1); // code 1 : erreur parsing
        }
    }
}
