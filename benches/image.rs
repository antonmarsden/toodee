//! Compare common row-major image workloads using `image::ImageBuffer` and `TooDee`.
//!
//! Run with:
//!   cargo bench --bench image
//!
//! These are deliberately separate from the library's normal dependencies: `image` is
//! a dev-dependency used only by this benchmark. The two containers hold the same
//! row-major u8 grayscale samples. The ImageBuffer cases use its public row/pixel APIs;
//! TooDee cases use row slices and 2D indexing.
//!
//! Results are workload-specific. In particular, these microbenchmarks do not establish
//! that either container is universally faster.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use image::{ImageBuffer, Luma};
use toodee::{TooDee, TooDeeOps, TooDeeOpsMut};

type GrayImage = ImageBuffer<Luma<u8>, Vec<u8>>;

fn make_samples(width: usize, height: usize) -> Vec<u8> {
    (0..width * height)
        .map(|i| ((i.wrapping_mul(37) + i / width.max(1) * 13) & 0xff) as u8)
        .collect()
}

fn image_buffer(width: usize, height: usize) -> GrayImage {
    ImageBuffer::from_raw(width as u32, height as u32, make_samples(width, height)).unwrap()
}

fn toodee(width: usize, height: usize) -> TooDee<u8> {
    TooDee::from_vec(width, height, make_samples(width, height))
}

// A row-major streaming pass, comparable to a common point operation. The
// ImageBuffer side uses rows_mut(); TooDee uses rows_mut().
fn row_invert(c: &mut Criterion) {
    let mut group = c.benchmark_group("image/row_invert");
    for (width, height) in [(320usize, 240usize), (1920, 1080), (3840, 2160)] {
        let elements = width * height;
        group.throughput(Throughput::Elements(elements as u64));

        let mut img = image_buffer(width, height);
        group.bench_with_input(BenchmarkId::new("ImageBuffer", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                for row in img.rows_mut() {
                    for pixel in row {
                        pixel.0[0] = !pixel.0[0];
                    }
                }
                black_box(img.as_raw());
            });
        });

        let mut td = toodee(width, height);
        group.bench_with_input(BenchmarkId::new("TooDee", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                for row in td.rows_mut() {
                    for pixel in row {
                        *pixel = !*pixel;
                    }
                }
                black_box(td.data());
            });
        });
    }
    group.finish();
}

// Read-only row traversal. This isolates the row iterator/data access pattern
// from arithmetic and allocation.
fn row_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("image/row_sum");
    for (width, height) in [(320usize, 240usize), (1920, 1080), (3840, 2160)] {
        let elements = width * height;
        group.throughput(Throughput::Elements(elements as u64));

        let img = image_buffer(width, height);
        group.bench_with_input(BenchmarkId::new("ImageBuffer", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                let sum: u64 = img.rows().flat_map(|row| row).map(|p| p.0[0] as u64).sum();
                black_box(sum);
            });
        });

        let td = toodee(width, height);
        group.bench_with_input(BenchmarkId::new("TooDee", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                let sum: u64 = td.rows().flat_map(|row| row.iter()).map(|p| *p as u64).sum();
                black_box(sum);
            });
        });
    }
    group.finish();
}

// A simple 3x3 neighbourhood average. The ImageBuffer version uses
// GenericImageView::get_pixel and GenericImage::put_pixel, a common generic
// image-crate style; TooDee uses direct 2D indexing. Edges are copied unchanged.
fn box_blur_3x3(c: &mut Criterion) {
    let mut group = c.benchmark_group("image/box_blur_3x3");
    for (width, height) in [(320usize, 240usize), (1920, 1080)] {
        let elements = width * height;
        group.throughput(Throughput::Elements(elements as u64));

        let src = image_buffer(width, height);
        let mut dst = GrayImage::new(width as u32, height as u32);
        group.bench_with_input(BenchmarkId::new("ImageBuffer_get_put", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                for y in 0..height {
                    for x in 0..width {
                        let value = if x == 0 || y == 0 || x + 1 == width || y + 1 == height {
                            src.get_pixel(x as u32, y as u32).0[0]
                        } else {
                            let mut sum = 0u32;
                            for dy in -1isize..=1 {
                                for dx in -1isize..=1 {
                                    sum += src.get_pixel((x as isize + dx) as u32, (y as isize + dy) as u32).0[0] as u32;
                                }
                            }
                            (sum / 9) as u8
                        };
                        dst.put_pixel(x as u32, y as u32, Luma([value]));
                    }
                }
                black_box(dst.as_raw());
            });
        });

        let src_raw = src.as_raw();
        let mut raw_dst = GrayImage::new(width as u32, height as u32);
        group.bench_with_input(BenchmarkId::new("ImageBuffer_raw_slice", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                let dst_raw = raw_dst.as_mut();
                for y in 0..height {
                    for x in 0..width {
                        let index = y * width + x;
                        dst_raw[index] = if x == 0 || y == 0 || x + 1 == width || y + 1 == height {
                            src_raw[index]
                        } else {
                            let mut sum = 0u32;
                            for dy in -1isize..=1 {
                                for dx in -1isize..=1 {
                                    let neighbour = ((y as isize + dy) as usize) * width
                                        + (x as isize + dx) as usize;
                                    sum += src_raw[neighbour] as u32;
                                }
                            }
                            (sum / 9) as u8
                        };
                    }
                }
                black_box(raw_dst.as_raw());
            });
        });

        let td_src = toodee(width, height);
        let mut td_dst = TooDee::new(width, height);
        group.bench_with_input(BenchmarkId::new("TooDee_index", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                for y in 0..height {
                    for x in 0..width {
                        td_dst[(x, y)] = if x == 0 || y == 0 || x + 1 == width || y + 1 == height {
                            td_src[(x, y)]
                        } else {
                            let mut sum = 0u32;
                            for dy in -1isize..=1 {
                                for dx in -1isize..=1 {
                                    sum += td_src[((x as isize + dx) as usize, (y as isize + dy) as usize)] as u32;
                                }
                            }
                            (sum / 9) as u8
                        };
                    }
                }
                black_box(td_dst.data());
            });
        });
    }
    group.finish();
}

// Explicitly measure the cost of crossing the representation boundary.
// ImageBuffer::into_raw and TooDee::from_vec can move the allocation without
// copying; converting back from a borrowed TooDee slice requires a copy.
fn representation_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("image/representation_conversion");
    for (width, height) in [(320usize, 240usize), (1920, 1080)] {
        let elements = width * height;
        group.throughput(Throughput::Elements(elements as u64));

        group.bench_with_input(BenchmarkId::new("ImageBuffer_to_TooDee_move", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                let img = image_buffer(width, height);
                let raw = img.into_raw();
                let td = TooDee::from_vec(width, height, raw);
                black_box(td);
            });
        });

        let td = toodee(width, height);
        group.bench_with_input(BenchmarkId::new("TooDee_to_ImageBuffer_copy", format!("{width}x{height}")), &elements, |b, _| {
            b.iter(|| {
                let img = GrayImage::from_raw(width as u32, height as u32, td.data().to_vec()).unwrap();
                black_box(img);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, row_invert, row_sum, box_blur_3x3, representation_conversion);
criterion_main!(benches);
