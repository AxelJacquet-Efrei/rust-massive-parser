use criterion::{criterion_group, criterion_main, Criterion};
use json_parser::JsonParser;
use std::io::Write;
use tempfile::NamedTempFile;

fn bench_parse_jsonl(c: &mut Criterion) {
    let mut file = NamedTempFile::new().unwrap();
    for i in 0..10_000 {
        writeln!(file, "{{\"idx\":{}}}", i).unwrap();
    }
    let path = file.path().to_path_buf();
    c.bench_function("parse_jsonl_10k", |b| {
        b.iter(|| {
            let doc = JsonParser::parse_jsonl_parallel(path.as_path()).unwrap();
            assert_eq!(doc.len(), 10_000);
        })
    });
    c.bench_function("parse_jsonl_simd_10k", |b| {
        b.iter(|| {
            let doc = JsonParser::parse_jsonl_parallel_simd(path.as_path()).unwrap();
            assert_eq!(doc.len(), 10_000);
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
    c.bench_function("parse_json_array_10k_auto", |b| {
        b.iter(|| {
            let doc = JsonParser::parse(path.as_path()).unwrap();
            assert_eq!(doc.len(), 10_000);
        })
    });
    c.bench_function("parse_json_array_10k_streaming", |b| {
        b.iter(|| {
            let arr: Vec<serde_json::Value> = JsonParser::parse_streaming(path.as_path()).unwrap();
            assert_eq!(arr.len(), 10_000);
        })
    });
    c.bench_function("parse_json_array_10k_simd", |b| {
        b.iter(|| {
            let doc = JsonParser::parse_simd(path.as_path()).unwrap();
            assert_eq!(doc.len(), 1); // simd-json retourne un tableau unique
        })
    });
}

criterion_group!(benches, bench_parse_jsonl, bench_parse_json_array);
criterion_main!(benches);
