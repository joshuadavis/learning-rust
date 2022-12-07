use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use circular_buffer1::VecCircBuf;

fn do_add(count: i32) {
    let mut buf = VecCircBuf::new(1024);
    for _i in 0..count {
        for s in 0..256 {
            buf.add(s);
        }
    }
}

fn do_add_iterate(count: i32) {
    let mut buf = VecCircBuf::new(1024);

    let mut z = 0;

    for _i in 0..count {
        for s in 0..256 {
            buf.add(s);
            for x in buf.iter() {
                let y = x * 2;
                z = z + y;
            }
        }
    }
}

fn bench_add(c: &mut Criterion) {
    let mut add_group = c.benchmark_group("add");
    for count in [1000, 2000, 3000, 4000].iter() {
        add_group.bench_with_input(
            BenchmarkId::from_parameter(count),
            count,
             |b, &count| {
                                       b.iter(|| { do_add(count) })
                                   }
        );
    }
    add_group.finish();
}

fn bench_add_iterate(c: &mut Criterion) {
    let mut add_group = c.benchmark_group("add_iterate");
    for count in [1000, 2000, 3000, 4000].iter() {
        add_group.bench_with_input(
            BenchmarkId::from_parameter(count),
            count,
            |b, &count| {
                b.iter(|| { do_add_iterate(count) })
            }
        );
    }
    add_group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .measurement_time(Duration::from_secs(20));
    targets = bench_add, bench_add_iterate
}

criterion_main!(benches);