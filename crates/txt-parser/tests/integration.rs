use parser_core::{Document, DocumentParser};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use txt_parser::TxtParser;

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
    assert_eq!(
        doc.lines().collect::<Vec<_>>(),
        vec!["ligne1", "ligne2", "ligne3"]
    );
}

#[test]
fn test_streaming_lines() {
    let data = b"a\nb\nc";
    let lines: Vec<_> = Document::streaming_lines(data)
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(lines, vec!["a", "b", "c"]);
}

#[test]
fn test_parse_txt_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "ligne1").unwrap();
    writeln!(file, "ligne2").unwrap();
    writeln!(file, "ligne3").unwrap();
    drop(file);
    let expected = vec!["ligne1", "ligne2", "ligne3"];
    let doc = TxtParser::parse(&file_path).unwrap();
    let lines: Vec<_> = doc.lines().collect();
    assert_eq!(lines, expected);
}

#[test]
fn test_parse_txt_empty() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("empty.txt");
    File::create(&file_path).unwrap();
    let doc = TxtParser::parse(&file_path).unwrap();
    assert_eq!(doc.line_count(), 0);
}

#[test]
fn test_parse_txt_malformed_utf8() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("bad.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"ligne1\nligne2\n\xFF\xFF\xFF").unwrap();
    drop(file);
    let res = TxtParser::parse(&file_path);
    assert!(res.is_err());
}

#[test]
fn test_parse_txt_file_not_found() {
    let path = std::path::Path::new("/tmp/__fichier_inexistant__.txt");
    let res = TxtParser::parse(path);
    assert!(res.is_err());
}
