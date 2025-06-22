use json_parser::JsonParser;
use parser_core::Document;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_parse_jsonl() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{{\"a\":1}}\n{{\"b\":2}}\n{{\"c\":3}}\n").unwrap();
    // Teste la stratégie auto (API utilisateur)
    let doc: Document = JsonParser::parse_as_document(file.path()).unwrap();
    assert_eq!(doc.line_count(), 3);
    let lines: Vec<_> = doc.lines().collect();
    assert!(lines[0].contains("\"a\":1"));
    assert!(lines[1].contains("\"b\":2"));
    assert!(lines[2].contains("\"c\":3"));
    // Teste explicitement la méthode JSONL SIMD (robustesse interne)
    let doc_vec = JsonParser::parse_jsonl_parallel_simd(file.path()).unwrap();
    assert_eq!(doc_vec.len(), 3);
    assert!(doc_vec.iter().any(|v| v.to_string().contains("\"a\":1")));
    assert!(doc_vec.iter().any(|v| v.to_string().contains("\"b\":2")));
    assert!(doc_vec.iter().any(|v| v.to_string().contains("\"c\":3")));
}

#[test]
fn test_parse_json_array() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "[{{\"a\":1}},{{\"b\":2}},{{\"c\":3}}]").unwrap();
    let doc: Document = JsonParser::parse_as_document(file.path()).unwrap();
    assert_eq!(doc.line_count(), 3);
    let lines: Vec<_> = doc.lines().collect();
    assert!(lines[0].contains("\"a\":1"));
    assert!(lines[1].contains("\"b\":2"));
    assert!(lines[2].contains("\"c\":3"));
}
