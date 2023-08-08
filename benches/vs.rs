use criterion::{criterion_group, criterion_main, Criterion};
use toodee::TooDee;
use grid::Grid;
use rand;
use rand::Rng;

const SIZE: usize = 1_000;

fn init_vec_vec() -> Vec<Vec<u8>> {
    vec![vec![0; SIZE]; SIZE]
}

fn init_grid() -> Grid<u8> {
    Grid::init(SIZE, SIZE, 0)
}

fn init_toodee() -> TooDee<u8> {
    TooDee::init(SIZE, SIZE, 0)
}

fn create_rand_ranged() -> usize {
    rand::thread_rng().gen_range(0..SIZE)
}

fn create_rand_tuple() -> (usize, usize) {
    let mut rnd = rand::thread_rng();
    (rnd.gen_range(0..SIZE), rnd.gen_range(0..SIZE))
}

fn bench_idx(c: &mut Criterion) {
    let mut group = c.benchmark_group("idx");
    group.bench_function("vecvec", |b| {
        let vec_vec = init_vec_vec();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| {
                let _v = vec_vec[x][y];
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| {
                let _v = grid[x][y];
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| {
                let _v = toodee[x][y];
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    group.bench_function("vecvec", |b| {
        let vec_vec = init_vec_vec();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| {
                let _v = vec_vec.get(x).unwrap().get(y).unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| {
                let _v = grid.get(x, y).unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| {
                let _v = toodee[(x,y)];
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("set");
    group.bench_function("vecvec", |b| {
        let mut vec_vec = init_vec_vec();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| vec_vec[x][y] = 42,
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("grid", |b| {
        let mut g = init_grid();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| g[x][y] = 42,
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let mut toodee = init_toodee();
        b.iter_batched(
            || create_rand_tuple(),
            |(x, y)| toodee[x][y] = 42,
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_push_row(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_row");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || (grid.clone(), vec![0; SIZE]),
            |(mut g, r) | g.push_row(r),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || (toodee.clone(), vec![0; SIZE]),
            |(mut g, r) | g.push_row(r),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_push_col(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_col");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || (grid.clone(), vec![0; SIZE]),
            |(mut g, c) | g.push_col(c),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || (toodee.clone(), vec![0; SIZE]),
            |(mut g, c) | g.push_row(c),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_pop_row(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_row");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || grid.clone(),
            |mut g | { g.pop_row(); },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || toodee.clone(),
            |mut g | { g.pop_row(); },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_pop_col(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_col");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || grid.clone(),
            |mut g | { g.pop_col(); },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || toodee.clone(),
            |mut g | { g.pop_col(); },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_insert_row(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_row");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || (grid.clone(), create_rand_ranged(), vec![0; SIZE]),
            |(mut g, idx, data) | { g.insert_row(idx, data); },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || (toodee.clone(), create_rand_ranged(), vec![0; SIZE]),
            |(mut g, idx, data) | { g.insert_row(idx, data); },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_insert_col(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_col");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || (grid.clone(), create_rand_ranged(), vec![0; SIZE]),
            |(mut g, idx, data) | { g.insert_col(idx, data); },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || (toodee.clone(), create_rand_ranged(), vec![0; SIZE]),
            |(mut g, idx, data) | { g.insert_col(idx, data); },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_remove_row(c: &mut Criterion) {
    let mut group = c.benchmark_group("remove_row");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || (grid.clone(), create_rand_ranged()),
            |(mut g, idx) | { g.remove_row(idx); },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || (toodee.clone(), create_rand_ranged()),
            |(mut g, idx) | { g.remove_row(idx); },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_remove_col(c: &mut Criterion) {
    let mut group = c.benchmark_group("remove_col");
    group.bench_function("grid", |b| {
        let grid = init_grid();
        b.iter_batched(
            || (grid.clone(), create_rand_ranged()),
            |(mut g, idx) | { g.remove_col(idx); },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("toodee", |b| {
        let toodee = init_toodee();
        b.iter_batched(
            || (toodee.clone(), create_rand_ranged()),
            |(mut g, idx) | { g.remove_col(idx); },
            criterion::BatchSize::SmallInput,
        )
    });
}



// criterion_group!(benches, bench_idx, bench_get, bench_set, bench_push_row);
criterion_group!(benches, bench_idx, bench_get, bench_set,
    bench_push_row, bench_push_col,
    bench_pop_row, bench_pop_col,
    bench_insert_row, bench_insert_col,
    bench_remove_row, bench_remove_col);

criterion_main!(benches);