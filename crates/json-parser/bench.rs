use criterion::{criterion_group, criterion_main, Criterion};
use json_parser::JsonParser;
use std::io::Write;
use tempfile::NamedTempFile;

fn bench_parse_jsonl(c: &mut Criterion) {
    let mut file = NamedTempFile::new().unwrap();
    // Génère 10_000 objets JSONL
    for i in 0..10_000 {
        writeln!(file, "{{\"idx\":{}}}", i).unwrap();
    }
    let path = file.path().to_path_buf();
    c.bench_function("parse_jsonl_10k", |b| {
        b.iter(|| {
            let doc = JsonParser::parse_as_document(&path).unwrap();
            assert_eq!(doc.line_count(), 10_000);
        })
    });
}

fn bench_parse_json_array(c: &mut Criterion) {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "[").unwrap();
    for i in 0..10_000 {
        if i > 0 { write!(file, ",").unwrap(); }
        write!(file, "{{\"idx\":{}}}", i).unwrap();
    }
    write!(file, "]").unwrap();
    let path = file.path().to_path_buf();
    c.bench_function("parse_json_array_10k", |b| {
        b.iter(|| {
            let doc = JsonParser::parse_as_document(&path).unwrap();
            assert_eq!(doc.line_count(), 10_000);
        })
    });
}

criterion_group!(benches, bench_parse_jsonl, bench_parse_json_array);
criterion_main!(benches);
