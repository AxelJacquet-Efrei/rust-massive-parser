use csv_parser::CsvParser;
use parser_core::DocumentParser;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_parse_csv() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a,b,c\n1,2,3\n4,5,6").unwrap();
    let doc = CsvParser::parse(file.path()).unwrap();
    assert_eq!(doc.line_count(), 3);
    let lines: Vec<_> = doc.lines().collect();
    assert!(lines[0].contains("a,b,c"));
    assert!(lines[1].contains("1,2,3"));
    assert!(lines[2].contains("4,5,6"));
}

#[test]
fn test_parse_tsv() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a\tb\tc\n1\t2\t3\n4\t5\t6").unwrap();
    let doc = CsvParser::parse(file.path()).unwrap();
    assert_eq!(doc.line_count(), 3);
    let lines: Vec<_> = doc.lines().collect();
    assert!(lines[0].contains("a\tb\tc"));
    assert!(lines[1].contains("1\t2\t3"));
    assert!(lines[2].contains("4\t5\t6"));
}

#[test]
fn test_parse_csv_exact() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a,b,c\n1,2,3\n4,5,6").unwrap();
    let expected = vec!["a,b,c", "1,2,3", "4,5,6"];
    let doc = CsvParser::parse(file.path()).unwrap();
    let lines: Vec<_> = doc.lines().collect();
    assert_eq!(lines, expected);
}

#[test]
fn test_parse_csv_empty() {
    let file = NamedTempFile::new().unwrap();
    let doc = CsvParser::parse(file.path()).unwrap();
    assert_eq!(doc.line_count(), 0);
}

#[test]
fn test_parse_csv_malformed_utf8() {
    let mut file = NamedTempFile::new().unwrap();
    let bad_bytes = b"a,b,c\n1,2,
\xFF\xFF\xFF";
    file.write_all(bad_bytes).unwrap();
    let res = CsvParser::parse(file.path());
    assert!(res.is_err());
}

#[test]
fn test_parse_csv_file_not_found() {
    let path = std::path::Path::new("/tmp/__fichier_inexistant__.csv");
    let res = CsvParser::parse(path);
    assert!(res.is_err());
}
