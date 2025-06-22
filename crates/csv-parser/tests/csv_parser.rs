use csv_parser::CsvParser;
use parser_core::DocumentParser;
use tempfile::NamedTempFile;
use std::io::Write;

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
