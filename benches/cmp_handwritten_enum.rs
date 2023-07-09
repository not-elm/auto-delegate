use criterion::{criterion_group, criterion_main, Criterion};

use auto_delegate_impl::Delegate;

use crate::utils::{Calc, CalcAdd};

mod utils;

enum Hand {
    Add(CalcAdd),
}

impl Calc for Hand {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        match self {
            Self::Add(add) => add.calc(x1, x2),
        }
    }
}

#[derive(Delegate)]
#[to(Calc)]
enum Generated {
    Add(CalcAdd),
}

fn generate_vs_handwritten(c: &mut Criterion) {
    let mut g = c.benchmark_group("generate_vs_handwritten");

    g.bench_function("generate", |b| {
        b.iter(|| {
            let calc = Generated::Add(CalcAdd);
            for _ in 0..10 {
                criterion::black_box(calc.calc(3, 2));
            }
        })
    });

    g.bench_function("handwritten", |b| {
        b.iter(|| {
            let calc = Hand::Add(CalcAdd);
            for _ in 0..10 {
                criterion::black_box(calc.calc(3, 2));
            }
        })
    });
}

criterion_group!(benches, generate_vs_handwritten);
criterion_main!(benches);
