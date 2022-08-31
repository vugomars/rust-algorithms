use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn constant(arr: &mut Vec<&str>) -> String {
    arr[0].to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut arr = black_box(vec![
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
        "Vu", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
    ]);
    c.bench_function("constant test", |b| b.iter(|| constant(&mut arr)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
