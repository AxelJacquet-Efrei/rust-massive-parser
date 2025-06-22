// Benchmark d'exemple pour txt-parser
use criterion::{criterion_group, criterion_main, Criterion};
use parser_core::DocumentParser;
use std::path::Path;
use txt_parser::TxtParser;

fn bench_parse(c: &mut Criterion) {
    let path = Path::new("../../fichier_1GB.txt");
    c.bench_function("parse 1GB file", |b| {
        b.iter(|| {
            let _doc = TxtParser::parse(path).unwrap();
        })
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
