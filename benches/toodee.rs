use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion, Throughput};
use toodee::{TooDee, TooDeeOps, TooDeeOpsMut};

fn fill_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill");
    for dims in [(32usize, 20usize), (320, 200), (640, 480)].iter() {
        let size = dims.0 * dims.1;
        group.throughput(Throughput::Elements(size as u64));
        let mut toodee = TooDee::new(dims.0, dims.1, 0u32);
        
        group.bench_with_input(BenchmarkId::new("fill", size), &size, |b, _| {
            b.iter(|| toodee.fill(42));
        });

        let mut view = toodee.view_mut((0, 0), (dims.0, dims.1));

        group.bench_with_input(BenchmarkId::new("view_fill", size), &size, |b, _| {
            b.iter(|| view.fill(42))
        });
    }
}

fn iter_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_sum");
    for dims in [(32usize, 20usize), (320, 200), (640, 480)].iter() {
        let size = dims.0 * dims.1;
        group.throughput(Throughput::Elements(size as u64));
        let mut toodee = TooDee::new(dims.0, dims.1, 1u32);
        group.bench_with_input(BenchmarkId::new("data", size), &size, |b, _| {
            b.iter(|| black_box(toodee.data().iter().sum::<u32>()));
        });
        let view = toodee.view_mut((0, 0), (dims.0, dims.1));
        group.bench_with_input(BenchmarkId::new("cells", size), &size, |b, _| {
            b.iter(|| black_box(view.cells().sum::<u32>()));
        });
        group.bench_with_input(BenchmarkId::new("rows", size), &size, |b, _| {
            b.iter(|| black_box(view.rows().map(|r| r.iter().sum::<u32>()).sum::<u32>()));
        });
    }
}

fn iter_mut_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_mut_sum");
    for dims in [(32usize, 20usize), (320, 200), (640, 480)].iter() {
        let size = dims.0 * dims.1;
        group.throughput(Throughput::Elements(size as u64));
        let mut toodee = TooDee::new(dims.0, dims.1, 1u32);
        group.bench_with_input(BenchmarkId::new("data_mut", size), &size, |b, _| {
            b.iter(|| black_box(toodee.data_mut().iter().sum::<u32>()));
        });
        let mut view = toodee.view_mut((0, 0), (dims.0, dims.1));
        group.bench_with_input(BenchmarkId::new("cells_mut", size), &size, |b, _| {
            b.iter(|| black_box(view.cells_mut().map(|x| *x).sum::<u32>()));
        });
        group.bench_with_input(BenchmarkId::new("rows_mut", size), &size, |b, _| {
            b.iter(|| black_box(view.rows_mut().map(|r| r.iter_mut().map(|x| *x).sum::<u32>()).sum::<u32>()));
        });
    }
}

criterion_group!(benches, fill_benchmark, iter_benchmark, iter_mut_benchmark);
criterion_main!(benches);
