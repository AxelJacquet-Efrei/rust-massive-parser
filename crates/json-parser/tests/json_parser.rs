use json_parser::JsonParser;
use parser_core::Document;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_parse_jsonl() {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{{\"a\":1}}\n{{\"b\":2}}\n{{\"c\":3}}\n").unwrap();
    let doc: Document = JsonParser::parse_as_document(file.path()).unwrap();
    assert_eq!(doc.line_count(), 3);
    let lines: Vec<_> = doc.lines().collect();
    assert!(lines[0].contains("\"a\":1"));
    assert!(lines[1].contains("\"b\":2"));
    assert!(lines[2].contains("\"c\":3"));
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
