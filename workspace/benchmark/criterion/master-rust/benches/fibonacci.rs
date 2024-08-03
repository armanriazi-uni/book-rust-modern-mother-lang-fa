#[macro_use]
extern crate criterion;

extern crate master_rust_benches_criterion_lib;

use master_rust_benches_criterion_lib::{slow_fibonacci, fast_fibonacci};

use criterion::Criterion;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 8", |b| b.iter(|| fast_fibonacci(8)));
}

criterion_group!(fib_bench, fibonacci_benchmark);
criterion_main!(fib_bench);
