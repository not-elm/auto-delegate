use criterion::{Criterion, criterion_group, criterion_main};

fn bm1(c: &mut Criterion) {
    c.bench_function("My Answer", |b| b.iter(|| {
        println!("HELLO")
    }));
}

fn bm2(c: &mut Criterion) {
    c.bench_function("Comp Answer", |b| b.iter(|| {}));
}

criterion_group!(benches, bm1, bm2);
criterion_main!(benches);