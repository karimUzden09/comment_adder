use criterion::{criterion_group, criterion_main, Criterion};
use comment_adder::app::banch_core::*;



pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add_comment_v2_bench_new", |b| b.iter(|| add_comment_v2_bench(&"E:/rust_projects/comment_adder/tt","E:/rust_projects/comment_adder/target/release/settings.json")));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);