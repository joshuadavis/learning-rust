use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use circular_buffer1::VecCircBuf;

fn add_and_iterate(count: i32) {
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

fn bench(c: &mut Criterion) {
    let mut add_group = c.benchmark_group("add");
    for count in [10000, 20000, 30000, 40000].iter() {
        add_group.bench_with_input(
            BenchmarkId::from_parameter(count),
            count,
             |b, &count| {
                                       b.iter(|| { add_and_iterate(count) })
                                   }
        );
    }
    add_group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);