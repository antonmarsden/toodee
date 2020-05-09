use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion, Throughput};
use toodee::{TooDee, TooDeeOpsMut, TranslateOps};

fn translate_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("translate");
    for dims in [(32usize, 20usize), (320, 200), (640, 480)].iter() {
        let size = dims.0 * dims.1;
        group.throughput(Throughput::Elements(size as u64));
        let mut toodee = TooDee::new(dims.0, dims.1, 0u32);
        
        group.bench_with_input(BenchmarkId::new("with_wrap", size), &size, |b, _| {
            b.iter(|| toodee.translate_with_wrap(3, 3))
        });

        group.bench_with_input(BenchmarkId::new("with_fill", size), &size, |b, _| {
            b.iter(|| toodee.translate_with_fill(3, 3, &42u32))
        });
        
        let mut view = toodee.view_mut(0, 0, dims.0, dims.1);

        group.bench_with_input(BenchmarkId::new("view_with_wrap", size), &size, |b, _| {
            b.iter(|| view.translate_with_wrap(3, 3))
        });

        group.bench_with_input(BenchmarkId::new("view_with_fill", size), &size, |b, _| {
            b.iter(|| view.translate_with_fill(3, 3, &42u32))
        });
    }
}

criterion_group!(benches, translate_benchmark);
criterion_main!(benches);
