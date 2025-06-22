// Benchmarks pour le parser CSV/TSV performant
// Utilisation : cargo bench --bench csv_bench

use criterion::{criterion_group, criterion_main, Criterion};
use csv_parser::CsvParser;
use parser_core::DocumentParser;
use std::path::Path;

fn bench_csv_parse(c: &mut Criterion) {
    let path = Path::new("../../fichier_1GB.txt"); // Remplacer par un vrai CSV/TSV pour test réel
    c.bench_function("csv_parse", |b| {
        b.iter(|| {
            let _ = CsvParser::parse(path).unwrap();
        });
    });
}

criterion_group!(benches, bench_csv_parse);
criterion_main!(benches);
