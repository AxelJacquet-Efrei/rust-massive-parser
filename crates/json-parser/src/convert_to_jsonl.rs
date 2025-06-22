use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let input_path = "/home/axel-jcqt/Documents/parser/fichier_1GB.json";
    let output_path = "/home/axel-jcqt/Documents/parser/fichier_1GB.jsonl";
    let input = File::open(input_path)?;
    let reader = BufReader::new(input);
    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);

    // On suppose que le fichier est un tableau JSON massif
    let values: Value = serde_json::from_reader(reader)?;
    if let Value::Array(arr) = values {
        for obj in &arr {
            let line = serde_json::to_string(obj)?;
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        println!("Conversion terminée : {} objets écrits dans {}", arr.len(), output_path);
    } else {
        println!("Le fichier d'entrée n'est pas un tableau JSON.");
    }
    Ok(())
}
