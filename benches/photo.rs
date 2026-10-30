use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use image::{ImageBuffer, Luma};
use toodee::TooDee;

type GrayImage = ImageBuffer<Luma<u8>, Vec<u8>>;

const SIZES: &[(usize, usize)] = &[(320, 240), (1920, 1080), (3840, 2160)];

fn make_pixels(width: usize, height: usize) -> Vec<u8> {
    // Deterministic, non-uniform input; avoid timing fixture generation.
    (0..width * height)
        .map(|i| ((i.wrapping_mul(37) ^ (i / width).wrapping_mul(19)) & 0xff) as u8)
        .collect()
}

fn image_from_pixels(width: usize, height: usize, pixels: &[u8]) -> GrayImage {
    ImageBuffer::from_raw(width as u32, height as u32, pixels.to_vec()).unwrap()
}

fn toodee_from_pixels(width: usize, height: usize, pixels: &[u8]) -> TooDee<u8> {
    TooDee::from_vec(width, height, pixels.to_vec())
}

// 3x3 Gaussian kernel [1, 2, 1] x [1, 2, 1], divisor 16.
// The outermost border is zeroed in both implementations.
fn gaussian_image(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    dst.fill(0);
    if w < 3 || h < 3 { return; }
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let i = y * w + x;
            let sum = src[i - w - 1] as u32
                + 2 * src[i - w] as u32
                + src[i - w + 1] as u32
                + 2 * src[i - 1] as u32
                + 4 * src[i] as u32
                + 2 * src[i + 1] as u32
                + src[i + w - 1] as u32
                + 2 * src[i + w] as u32
                + src[i + w + 1] as u32;
            dst[i] = ((sum + 8) / 16) as u8;
        }
    }
}

fn gaussian_toodee(src: &TooDee<u8>, dst: &mut TooDee<u8>) {
    let (w, h) = (src.num_cols(), src.num_rows());
    for row in dst.rows_mut() { row.fill(0); }
    if w < 3 || h < 3 { return; }
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let sum = src[y - 1][x - 1] as u32
                + 2 * src[y - 1][x] as u32
                + src[y - 1][x + 1] as u32
                + 2 * src[y][x - 1] as u32
                + 4 * src[y][x] as u32
                + 2 * src[y][x + 1] as u32
                + src[y + 1][x - 1] as u32
                + 2 * src[y + 1][x] as u32
                + src[y + 1][x + 1] as u32;
            dst[y][x] = ((sum + 8) / 16) as u8;
        }
    }
}

fn unsharp_image(src: &[u8], blur: &mut [u8], dst: &mut [u8], w: usize, h: usize) {
    gaussian_image(src, blur, w, h);
    for i in 0..src.len() {
        dst[i] = (2 * src[i] as i16 - blur[i] as i16).clamp(0, 255) as u8;
    }
}

fn unsharp_toodee(src: &TooDee<u8>, blur: &mut TooDee<u8>, dst: &mut TooDee<u8>) {
    gaussian_toodee(src, blur);
    for y in 0..src.num_rows() {
        for x in 0..src.num_cols() {
            dst[y][x] = (2 * src[y][x] as i16 - blur[y][x] as i16).clamp(0, 255) as u8;
        }
    }
}

// Sobel edge strength is |Gx| + |Gy|, saturated to u8.
// Border pixels are zeroed.
fn sobel_image(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    dst.fill(0);
    if w < 3 || h < 3 { return; }
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let i = y * w + x;
            let tl = src[i - w - 1] as i32;
            let tc = src[i - w] as i32;
            let tr = src[i - w + 1] as i32;
            let ml = src[i - 1] as i32;
            let mr = src[i + 1] as i32;
            let bl = src[i + w - 1] as i32;
            let bc = src[i + w] as i32;
            let br = src[i + w + 1] as i32;
            let gx = -tl + tr - 2 * ml + 2 * mr - bl + br;
            let gy = -tl - 2 * tc - tr + bl + 2 * bc + br;
            dst[i] = (gx.abs() + gy.abs()).min(255) as u8;
        }
    }
}

fn sobel_toodee(src: &TooDee<u8>, dst: &mut TooDee<u8>) {
    let (w, h) = (src.num_cols(), src.num_rows());
    for row in dst.iter_rows_mut() { row.fill(0); }
    if w < 3 || h < 3 { return; }
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let tl = src[y - 1][x - 1] as i32;
            let tc = src[y - 1][x] as i32;
            let tr = src[y - 1][x + 1] as i32;
            let ml = src[y][x - 1] as i32;
            let mr = src[y][x + 1] as i32;
            let bl = src[y + 1][x - 1] as i32;
            let bc = src[y + 1][x] as i32;
            let br = src[y + 1][x + 1] as i32;
            let gx = -tl + tr - 2 * ml + 2 * mr - bl + br;
            let gy = -tl - 2 * tc - tr + bl + 2 * bc + br;
            dst[y][x] = (gx.abs() + gy.abs()).min(255) as u8;
        }
    }
}

fn equalize_image(src: &[u8], dst: &mut [u8]) {
    let mut hist = [0u32; 256];
    for &p in src { hist[p as usize] += 1; }
    let mut cdf = [0u32; 256];
    let mut sum = 0u32;
    for i in 0..256 { sum += hist[i]; cdf[i] = sum; }
    let first = cdf.iter().copied().find(|&v| v != 0).unwrap_or(0);
    let total = src.len() as u32;
    if total == first { dst.copy_from_slice(src); return; }
    let denom = total - first;
    let mut lut = [0u8; 256];
    for i in 0..256 {
        lut[i] = if cdf[i] <= first { 0 } else {
            (((cdf[i] - first) as u64 * 255 + (denom / 2) as u64) / denom as u64) as u8
        };
    }
    for (d, &s) in dst.iter_mut().zip(src) { *d = lut[s as usize]; }
}

fn equalize_toodee(src: &TooDee<u8>, dst: &mut TooDee<u8>) {
    let mut hist = [0u32; 256];
    for &p in src.data() { hist[p as usize] += 1; }
    let mut cdf = [0u32; 256];
    let mut sum = 0u32;
    for i in 0..256 { sum += hist[i]; cdf[i] = sum; }
    let first = cdf.iter().copied().find(|&v| v != 0).unwrap_or(0);
    let total = src.data().len() as u32;
    if total == first {
        dst.data_mut().copy_from_slice(src.data());
        return;
    }
    let denom = total - first;
    let mut lut = [0u8; 256];
    for i in 0..256 {
        lut[i] = if cdf[i] <= first { 0 } else {
            (((cdf[i] - first) as u64 * 255 + (denom / 2) as u64) / denom as u64) as u8
        };
    }
    for (d, &s) in dst.data_mut().iter_mut().zip(src.data()) { *d = lut[s as usize]; }
}

fn photo_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("photo_processing");
    for &(w, h) in SIZES {
        let pixels = make_pixels(w, h);
        let n = w * h;
        group.throughput(Throughput::Elements(n as u64));

        let image_src = image_from_pixels(w, h, &pixels);
        let toodee_src = toodee_from_pixels(w, h, &pixels);

        group.bench_function(BenchmarkId::new("gaussian_3x3/imagebuffer", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || image_from_pixels(w, h, &vec![0; n]),
                |mut dst| {
                    gaussian_image(image_src.as_raw(), dst.as_mut(), w, h);
                    black_box(dst);
                },
                BatchSize::LargeInput,
            )
        });
        group.bench_function(BenchmarkId::new("gaussian_3x3/toodee", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || TooDee::init(w, h, 0u8),
                |mut dst| { gaussian_toodee(&toodee_src, &mut dst); black_box(dst); },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("unsharp_mask/imagebuffer", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || (vec![0; n], vec![0; n]),
                |(mut blur, mut dst)| {
                    unsharp_image(image_src.as_raw(), &mut blur, &mut dst, w, h);
                    black_box(dst);
                },
                BatchSize::LargeInput,
            )
        });
        group.bench_function(BenchmarkId::new("unsharp_mask/toodee", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || (TooDee::init(w, h, 0u8), TooDee::init(w, h, 0u8)),
                |(mut blur, mut dst)| {
                    unsharp_toodee(&toodee_src, &mut blur, &mut dst);
                    black_box(dst);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("sobel/imagebuffer", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || image_from_pixels(w, h, &vec![0; n]),
                |mut dst| {
                    sobel_image(image_src.as_raw(), dst.as_mut(), w, h);
                    black_box(dst);
                },
                BatchSize::LargeInput,
            )
        });
        group.bench_function(BenchmarkId::new("sobel/toodee", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || TooDee::init(w, h, 0u8),
                |mut dst| { sobel_toodee(&toodee_src, &mut dst); black_box(dst); },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("histogram_equalization/imagebuffer", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || image_from_pixels(w, h, &vec![0; n]),
                |mut dst| {
                    equalize_image(image_src.as_raw(), dst.as_mut());
                    black_box(dst);
                },
                BatchSize::LargeInput,
            )
        });
        group.bench_function(BenchmarkId::new("histogram_equalization/toodee", format!("{w}x{h}")), |b| {
            b.iter_batched(
                || TooDee::init(w, h, 0u8),
                |mut dst| { equalize_toodee(&toodee_src, &mut dst); black_box(dst); },
                BatchSize::LargeInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, photo_benchmarks);
criterion_main!(benches);
