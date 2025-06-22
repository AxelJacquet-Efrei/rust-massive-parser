use std::time::Instant;
use std::path::Path;
use json_parser::convert_to_jsonl_mod::convert_to_jsonl;
use json_parser::JsonParser;

fn main() {
    let input = Path::new("/home/axel-jcqt/Documents/parser/fichier_1GB.json");
    let output = Path::new("/home/axel-jcqt/Documents/parser/fichier_1GB.jsonl");

    let start = Instant::now();
    convert_to_jsonl(input, output).expect("Conversion JSON->JSONL échouée");
    let t_conv = start.elapsed();

    let start = Instant::now();
    let doc = JsonParser::parse_jsonl_parallel_simd(output).expect("Parsing JSONL échoué");
    let t_parse = start.elapsed();

    println!("Conversion: {:?}", t_conv);
    println!("Parsing JSONL: {:?} ({} objets)", t_parse, doc.len());
    println!("Total conversion + parsing: {:?}", t_conv + t_parse);
}
