use anyhow::Result;
use parser_core::DocumentParser;
use std::path::PathBuf;
use txt_parser::TxtParser;
use json_parser::JsonParser;
use std::time::Instant;

fn main() -> Result<()> {
    let path: PathBuf = std::env::args()
        .nth(1)
        .expect("Usage: parser-cli <fichier.txt|.json>")
        .into();

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "txt" => {
            let metadata = std::fs::metadata(&path)?;
            let file_size = metadata.len();
            let (mode, stride);
            let start = Instant::now();
            let doc = if file_size < 10 * 1024 * 1024 * 1024 {
                // < 10 Go : indexation complète
                mode = "complet";
                stride = 1;
                TxtParser::parse(&path)?
            } else if file_size < 100 * 1024 * 1024 * 1024 {
                // 10-100 Go : index partiel (1 ligne sur 10_000)
                mode = "partiel";
                stride = 10_000;
                TxtParser::parse_with_partial_index(&path, stride, false)?
            } else {
                // > 100 Go : index très partiel (1 ligne sur 100_000)
                mode = "partiel";
                stride = 100_000;
                TxtParser::parse_with_partial_index(&path, stride, false)?
            };
            let elapsed = start.elapsed();
            let nb_lignes = doc.offsets.len();
            let mem = procfs::process::Process::myself()?.statm()?.resident * 4096 / 1024;
            println!("Document chargé : {} lignes", nb_lignes);
            println!("Temps de parsing : {:.2?}", elapsed);
            println!("Mémoire utilisée (approx) : {} Ko", mem);
            println!("Mode d'indexation : {} (stride = {})", mode, stride);
            let first_line = doc.lines().next().map(|s| s.to_string());
            if let Some(first) = first_line {
                println!("Ligne 1 : {}", first);
            }
        }
        "json" => {
            let metadata = std::fs::metadata(&path)?;
            let file_size = metadata.len();
            let start = Instant::now();
            let values = JsonParser::parse_auto(&path)?;
            let elapsed = start.elapsed();
            let mem = procfs::process::Process::myself()?.statm()?.resident * 4096 / 1024;
            println!("JSON chargé : {} objets", values.len());
            println!("Temps de parsing : {:.2?}", elapsed);
            println!("Mémoire utilisée (approx) : {} Ko", mem);
            if file_size < 1 * 1024 * 1024 * 1024 {
                if let Some(first) = values.get(0) {
                    println!("Premier objet : {}", first);
                }
            } else {
                println!("Premier objet non affiché (fichier > 1 Go)");
            }
        }
        _ => {
            eprintln!("Extension non supportée : {}", ext);
            std::process::exit(1);
        }
    }
    Ok(())
}
