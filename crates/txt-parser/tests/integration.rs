use txt_parser::TxtParser;
use parser_core::{Document, DocumentParser};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_parse_and_access() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "ligne1").unwrap();
    writeln!(file, "ligne2").unwrap();
    writeln!(file, "ligne3").unwrap();
    drop(file);

    let doc = TxtParser::parse(&file_path).unwrap();
    assert_eq!(doc.line_count(), 3);
    assert_eq!(doc.get_line(1).unwrap(), "ligne2");
    assert_eq!(doc.lines().collect::<Vec<_>>(), vec!["ligne1", "ligne2", "ligne3"]);
}

#[test]
fn test_streaming_lines() {
    let data = b"a\nb\nc";
    let lines: Vec<_> = Document::streaming_lines(data)
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(lines, vec!["a", "b", "c"]);
}
