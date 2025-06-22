use criterion::{Criterion, criterion_group, criterion_main};
fn bench_trivial(c: &mut Criterion) {
    c.bench_function("trivial", |b| b.iter(|| 42));
}
criterion_group!(benches, bench_trivial);
criterion_main!(benches);
