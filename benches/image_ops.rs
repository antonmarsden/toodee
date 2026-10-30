use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use image::imageops;
use image::{GenericImageView, ImageBuffer, Luma};
use toodee::{CopyOps, TooDee, TooDeeOps, TooDeeOpsMut, TranslateOps};

type GrayImage = ImageBuffer<Luma<u8>, Vec<u8>>;

const SIZES: &[(usize, usize)] = &[(320, 240), (1920, 1080), (3840, 2160)];

fn pixels(width: usize, height: usize, seed: usize) -> Vec<u8> {
    (0..width * height)
        .map(|i| {
            ((i.wrapping_mul(37) ^ (i / width).wrapping_mul(19) ^ seed) & 0xff) as u8
        })
        .collect()
}

fn image_from_pixels(width: usize, height: usize, data: &[u8]) -> GrayImage {
    ImageBuffer::from_raw(width as u32, height as u32, data.to_vec()).unwrap()
}

fn toodee_from_pixels(width: usize, height: usize, data: &[u8]) -> TooDee<u8> {
    TooDee::from_vec(width, height, data.to_vec())
}

fn blank_image(width: usize, height: usize) -> GrayImage {
    ImageBuffer::from_raw(width as u32, height as u32, vec![0; width * height]).unwrap()
}

fn roi_sum_image(src: &GrayImage, x: usize, y: usize, width: usize, height: usize) -> u64 {
    let roi = imageops::crop_imm(src, x as u32, y as u32, width as u32, height as u32);
    let mut sum = 0u64;
    for row in 0..height {
        for col in 0..width {
            sum += u64::from(roi.get_pixel(col as u32, row as u32)[0]);
        }
    }
    sum
}

fn roi_sum_toodee(src: &TooDee<u8>, x: usize, y: usize, width: usize, height: usize) -> u64 {
    src.view((x, y), (x + width, y + height))
        .rows()
        .flat_map(|row| row.iter())
        .map(|&pixel| u64::from(pixel))
        .sum()
}

fn tile_toodee(src: &TooDee<u8>, dst: &mut TooDee<u8>) {
    let tile_width = src.num_cols();
    let tile_height = src.num_rows();
    for (y, dst_row) in dst.rows_mut().enumerate() {
        let src_row = &src[y % tile_height];
        for chunk in dst_row.chunks_mut(tile_width) {
            let len = chunk.len();
            chunk.copy_from_slice(&src_row[..len]);
        }
    }
}

fn translate_image_with_wrap(image: &mut GrayImage, x: usize, y: usize) {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let data = image.as_mut();
    data.rotate_left(y * width);
    for row in data.chunks_exact_mut(width) {
        row.rotate_left(x);
    }
    debug_assert_eq!(data.len(), width * height);
}

fn assert_equivalent_operations() {
    let (width, height) = (320, 240);
    let source_data = pixels(width, height, 0);
    let image = image_from_pixels(width, height, &source_data);
    let toodee = toodee_from_pixels(width, height, &source_data);

    // ROI traversal.
    assert_eq!(
        roi_sum_image(&image, 17, 23, 91, 77),
        roi_sum_toodee(&toodee, 17, 23, 91, 77)
    );

    // Rectangular replacement.
    let patch_data = pixels(73, 51, 11);
    let patch_image = image_from_pixels(73, 51, &patch_data);
    let patch_toodee = toodee_from_pixels(73, 51, &patch_data);
    let mut replaced_image = blank_image(width, height);
    let mut replaced_toodee = TooDee::init(width, height, 0u8);
    imageops::replace(&mut replaced_image, &patch_image, 29, 41);
    replaced_toodee
        .view_mut((29, 41), (29 + 73, 41 + 51))
        .copy_from_toodee(&patch_toodee);
    assert_eq!(replaced_image.as_raw(), replaced_toodee.data());

    // Tiling, including partial tiles at the right and bottom edges.
    let tile_data = pixels(64, 47, 29);
    let tile_image = image_from_pixels(64, 47, &tile_data);
    let tile_toodee_src = toodee_from_pixels(64, 47, &tile_data);
    let mut tiled_image = blank_image(width, height);
    let mut tiled_toodee = TooDee::init(width, height, 0u8);
    imageops::tile(&mut tiled_image, &tile_image);
    tile_toodee(&tile_toodee_src, &mut tiled_toodee);
    assert_eq!(tiled_image.as_raw(), tiled_toodee.data());

    // Wrapped translation and flips.
    let shift = (width / 3, height / 4);
    let mut translated_image = image.clone();
    let mut translated_toodee = toodee.clone();
    translate_image_with_wrap(&mut translated_image, shift.0, shift.1);
    translated_toodee.translate_with_wrap(shift);
    assert_eq!(translated_image.as_raw(), translated_toodee.data());

    let mut horizontal_image = image.clone();
    let mut horizontal_toodee = toodee.clone();
    imageops::flip_horizontal_in_place(&mut horizontal_image);
    horizontal_toodee.flip_cols();
    assert_eq!(horizontal_image.as_raw(), horizontal_toodee.data());

    let mut vertical_image = image;
    let mut vertical_toodee = toodee;
    imageops::flip_vertical_in_place(&mut vertical_image);
    vertical_toodee.flip_rows();
    assert_eq!(vertical_image.as_raw(), vertical_toodee.data());
}

fn image_ops_benchmarks(c: &mut Criterion) {
    assert_equivalent_operations();

    let mut group = c.benchmark_group("image_ops");

    for &(width, height) in SIZES {
        let count = width * height;
        let source_data = pixels(width, height, 0);
        let image_src = image_from_pixels(width, height, &source_data);
        let toodee_src = toodee_from_pixels(width, height, &source_data);
        group.throughput(Throughput::Elements(count as u64));

        // Read-only ROI traversal. The crop/view itself is included; no pixel
        // buffer is allocated by either implementation.
        let roi_width = (width / 4).max(32);
        let roi_height = (height / 4).max(32);
        let roi_x = (width - roi_width) / 2;
        let roi_y = (height - roi_height) / 2;
        group.throughput(Throughput::Elements((roi_width * roi_height) as u64));
        group.bench_function(
            BenchmarkId::new("roi_sum/image_crop_imm", format!("{width}x{height}")),
            |b| {
                b.iter(|| {
                    black_box(roi_sum_image(
                        &image_src,
                        roi_x,
                        roi_y,
                        roi_width,
                        roi_height,
                    ));
                })
            },
        );
        group.bench_function(
            BenchmarkId::new("roi_sum/toodee_view_rows", format!("{width}x{height}")),
            |b| {
                b.iter(|| {
                    black_box(roi_sum_toodee(
                        &toodee_src,
                        roi_x,
                        roi_y,
                        roi_width,
                        roi_height,
                    ));
                })
            },
        );

        group.throughput(Throughput::Elements(count as u64));

        // Replace a centred rectangular region with another image.
        let patch_width = (width / 4).max(32);
        let patch_height = (height / 4).max(32);
        let patch_x = (width - patch_width) / 2;
        let patch_y = (height - patch_height) / 2;
        let patch_data = pixels(patch_width, patch_height, 11);
        let patch_image = image_from_pixels(patch_width, patch_height, &patch_data);
        let patch_toodee = toodee_from_pixels(patch_width, patch_height, &patch_data);
        group.bench_function(
            BenchmarkId::new("replace/imageops", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || blank_image(width, height),
                    |mut dst| {
                        imageops::replace(
                            &mut dst,
                            &patch_image,
                            patch_x as i64,
                            patch_y as i64,
                        );
                        black_box(dst);
                    },
                    BatchSize::LargeInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("replace/toodee_view_copy", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || TooDee::init(width, height, 0u8),
                    |mut dst| {
                        dst.view_mut(
                            (patch_x, patch_y),
                            (patch_x + patch_width, patch_y + patch_height),
                        )
                        .copy_from_toodee(&patch_toodee);
                        black_box(dst);
                    },
                    BatchSize::LargeInput,
                )
            },
        );

        // Repeat a smaller source tile over the complete destination. Partial
        // tiles at image boundaries are part of the workload.
        let tile_width = 64;
        let tile_height = 47;
        let tile_data = pixels(tile_width, tile_height, 29);
        let tile_image = image_from_pixels(tile_width, tile_height, &tile_data);
        let tile_toodee_src = toodee_from_pixels(tile_width, tile_height, &tile_data);
        group.bench_function(
            BenchmarkId::new("tile/imageops", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || blank_image(width, height),
                    |mut dst| {
                        imageops::tile(&mut dst, &tile_image);
                        black_box(dst);
                    },
                    BatchSize::LargeInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("tile/toodee_rows", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || TooDee::init(width, height, 0u8),
                    |mut dst| {
                        tile_toodee(&tile_toodee_src, &mut dst);
                        black_box(dst);
                    },
                    BatchSize::LargeInput,
                )
            },
        );

        // Both paths perform the same in-place toroidal shift. The ImageBuffer
        // baseline uses its contiguous backing slice; TooDee uses its API.
        let shift_x = width / 3;
        let shift_y = height / 4;
        group.bench_function(
            BenchmarkId::new("translate_wrap/imagebuffer_slice", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || image_src.clone(),
                    |mut image| {
                        translate_image_with_wrap(&mut image, shift_x, shift_y);
                        black_box(image);
                    },
                    BatchSize::LargeInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("translate_wrap/toodee", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || toodee_src.clone(),
                    |mut image| {
                        image.translate_with_wrap((shift_x, shift_y));
                        black_box(image);
                    },
                    BatchSize::LargeInput,
                )
            },
        );

        // In-place flips.
        group.bench_function(
            BenchmarkId::new("flip_horizontal/imageops", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || image_src.clone(),
                    |mut image| {
                        imageops::flip_horizontal_in_place(&mut image);
                        black_box(image);
                    },
                    BatchSize::LargeInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("flip_horizontal/toodee", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || toodee_src.clone(),
                    |mut image| {
                        image.flip_cols();
                        black_box(image);
                    },
                    BatchSize::LargeInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("flip_vertical/imageops", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || image_src.clone(),
                    |mut image| {
                        imageops::flip_vertical_in_place(&mut image);
                        black_box(image);
                    },
                    BatchSize::LargeInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("flip_vertical/toodee", format!("{width}x{height}")),
            |b| {
                b.iter_batched(
                    || toodee_src.clone(),
                    |mut image| {
                        image.flip_rows();
                        black_box(image);
                    },
                    BatchSize::LargeInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(benches, image_ops_benchmarks);
criterion_main!(benches);
