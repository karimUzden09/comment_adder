use criterion::{criterion_group, criterion_main, Criterion};
use comment_adder::app::banch_core::*;



pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("paralel_scunn_filles_bench_one_core_12288_files", |b| b.iter(|| paralel_scan_files_bench(&"E:/rust_projects/comment_adder_t_poligon","E:/rust_projects/comment_adder/target/release/settings.json")));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);