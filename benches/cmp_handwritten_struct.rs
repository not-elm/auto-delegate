use criterion::{Criterion, criterion_group, criterion_main};

use auto_delegate_macros::Delegate;

use crate::utils::{Calc, CalcAdd};

mod utils;

#[derive(Default)]
struct HandWritten {
    child: CalcAdd,
}


impl Calc for HandWritten {
    fn calc(&self, x1: usize, x2: usize) -> usize {
        self.child.calc(x1, x2)
    }
}


#[derive(Delegate, Default)]
struct Generated {
    #[to(Calc)]
    child: CalcAdd,
}


fn generate_vs_handwritten(c: &mut Criterion) {
    let mut g = c.benchmark_group("generate_vs_handwritten");

    g.bench_function("generate", |b| b.iter(|| {
        let calc = Generated::default();
        for _ in 0..10 {
            criterion::black_box(calc.calc(3, 2));
        }
    }));


    g.bench_function("handwritten", |b| b.iter(|| {
        let calc = HandWritten::default();
        for _ in 0..10 {
            criterion::black_box(calc.calc(3, 2));
        }
    }));
}



criterion_group!(benches_struct, generate_vs_handwritten);
criterion_main!(benches_struct);