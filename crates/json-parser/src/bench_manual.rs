use std::time::Instant;
use json_parser::JsonParser;
use std::path::Path;

fn main() {
    let files = vec![
        ("JSON massif (tableau)", "/home/axel-jcqt/Documents/parser/fichier_1GB.json"),
        ("JSONL (une ligne = un objet)", "/home/axel-jcqt/Documents/parser/fichier_1GB.jsonl"),
    ];
    for (desc, path_str) in files {
        let path = Path::new(path_str);
        println!("\n=== Benchmark sur : {} ===", desc);
        let mut results = Vec::new();

        // Auto (meilleur choix)
        let start = Instant::now();
        let doc = JsonParser::parse_auto(path).unwrap();
        let dur = start.elapsed();
        results.push(("parse_auto (meilleur choix)", dur, doc.len()));

        // JSONL parallèle SIMD
        let start = Instant::now();
        let doc = JsonParser::parse_jsonl_parallel_simd(path).unwrap_or_default();
        let dur = start.elapsed();
        results.push(("parse_jsonl_parallel_simd", dur, doc.len()));

        // JSON array streaming
        let start = Instant::now();
        let arr: Vec<serde_json::Value> = JsonParser::parse_streaming(path).map(|v: serde_json::Value| if let serde_json::Value::Array(arr) = v { arr } else { vec![v] }).unwrap_or_default();
        let dur = start.elapsed();
        results.push(("parse_json_array_streaming", dur, arr.len()));

        // JSON array SIMD
        let start = Instant::now();
        let doc = JsonParser::parse_simd(path).unwrap_or_default();
        let dur = start.elapsed();
        results.push(("parse_json_array_simd", dur, doc.len()));

        println!("\nRésultats :");
        for (name, dur, len) in results {
            println!("{}: {:?} ({} éléments)", name, dur, len);
        }
    }
}
