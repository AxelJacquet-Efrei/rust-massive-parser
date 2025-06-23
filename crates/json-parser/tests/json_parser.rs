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

#[test]
fn test_parse_jsonl_exact() {
    use serde_json::json;
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{{\"a\":1}}\n{{\"b\":2}}\n{{\"c\":3}}\n").unwrap();
    let expected = vec![
        json!({"a": 1}),
        json!({"b": 2}),
        json!({"c": 3}),
    ];
    let doc_vec = JsonParser::parse_jsonl_parallel_simd(file.path()).unwrap();
    assert_eq!(doc_vec, expected);
}

#[test]
fn test_parse_json_array_exact() {
    use serde_json::json;
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "[{{\"a\":1}},{{\"b\":2}},{{\"c\":3}}]").unwrap();
    let expected = vec![
        json!({"a": 1}),
        json!({"b": 2}),
        json!({"c": 3}),
    ];
    let doc: Vec<serde_json::Value> = JsonParser::parse(file.path()).unwrap();
    // Si c'est un array unique, aplatit
    let parsed = if doc.len() == 1 && doc[0].is_array() {
        doc[0].as_array().unwrap().clone()
    } else {
        doc
    };
    assert_eq!(parsed, expected);
}

#[test]
fn test_parse_jsonl_empty() {
    let file = NamedTempFile::new().unwrap();
    let res = JsonParser::parse_jsonl_parallel_simd(file.path());
    assert!(res.is_ok());
    assert_eq!(res.unwrap().len(), 0);
}

#[test]
fn test_parse_jsonl_malformed() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{{not_json}}").unwrap();
    let res = JsonParser::parse_jsonl_parallel_simd(file.path());
    assert!(res.is_err());
}

#[test]
fn test_parse_json_file_not_found() {
    let path = std::path::Path::new("/tmp/__fichier_inexistant__.json");
    let res = JsonParser::parse(path);
    assert!(res.is_err());
}
