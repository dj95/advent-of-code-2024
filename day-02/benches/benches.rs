use day_02::{part_one, part_two, read_lines};

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_part_1(c: &mut Criterion) {
    let lines = read_lines();

    c.bench_function("part_1", |b| {
        b.iter(|| part_one(&lines));
    });
}

fn bench_part_2(c: &mut Criterion) {
    let lines = read_lines();

    c.bench_function("part_2", |b| {
        b.iter(|| part_two(&lines));
    });
}
fn criterion_benchmark(c: &mut Criterion) {
    bench_part_1(c);
    bench_part_2(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
