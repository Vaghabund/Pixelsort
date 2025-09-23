// random_sort.rs
// Random Exclude Pixel Sorting Funktionen

use rand::Rng;
use image;

pub fn brightness_f32(px: &image::Rgba<u8>) -> f32 {
    let [r, g, b, _] = px.0;
    (r as f32 + g as f32 + b as f32) / (3.0 * 255.0)
}

pub fn red_value_f32(px: &image::Rgba<u8>) -> f32 {
    px[0] as f32 / 255.0
}

pub fn inverted_red_f32(px: &image::Rgba<u8>) -> f32 {
    1.0 - (px[0] as f32 / 255.0)
}

// Group pixels probabilistically for sorting. `prob` is the probability (0.0..=1.0)
// that a pixel will be included in the sortable group. Pixels that are not
// selected are emitted as single-element groups and the currently collected
// group (if any) is flushed sorted.
pub fn random_exclude(
    pixels: Vec<image::Rgba<u8>>,
    sort_func: fn(&image::Rgba<u8>) -> f32,
    prob: f32,
) -> Vec<Vec<image::Rgba<u8>>> {
    let mut chunks: Vec<Vec<image::Rgba<u8>>> = Vec::new();
    let mut group: Vec<image::Rgba<u8>> = Vec::new();
    let mut rng = rand::thread_rng();

    for pixel in pixels {
        let include = if prob <= 0.0 { false } else if prob >= 1.0 { true } else { rng.gen_bool(prob as f64) };

        if include {
            // add to current sortable group
            group.push(pixel);
        } else {
            // flush any collected group (sorted) and emit the single pixel as its own group
            if !group.is_empty() {
                group.sort_by_key(|px| (sort_func(px) * 1000.0) as u32);
                chunks.push(std::mem::take(&mut group));
            }
            chunks.push(vec![pixel]);
        }
    }

    if !group.is_empty() {
        group.sort_by_key(|px| (sort_func(px) * 1000.0) as u32);
        chunks.push(group);
    }

    chunks
}

pub fn apply_random_exclude_to_row(
    img: &mut image::RgbaImage,
    y: u32,
    x_start: u32,
    x_end: u32,
    sort_func: fn(&image::Rgba<u8>) -> f32,
    prob: f32,
) {
    let pixels: Vec<_> = (x_start..x_end)
        .map(|x| *img.get_pixel(x, y))
        .collect();

    // Use configured probability for inclusion in sortable groups
    let chunks = random_exclude(pixels, sort_func, prob);

    let mut x_offset = x_start;
    for chunk in chunks {
        for pixel in chunk {
            if x_offset < x_end {
                img.put_pixel(x_offset, y, pixel);
                x_offset += 1;
            }
        }
    }
}

pub fn apply_random_exclude_to_column(
    img: &mut image::RgbaImage,
    x: u32,
    y_start: u32,
    y_end: u32,
    sort_func: fn(&image::Rgba<u8>) -> f32,
    prob: f32,
) {
    let pixels: Vec<_> = (y_start..y_end)
        .map(|y| *img.get_pixel(x, y))
        .collect();

    // Use configured probability for inclusion in sortable groups
    let chunks = random_exclude(pixels, sort_func, prob);

    let mut y_offset = y_start;
    for chunk in chunks {
        for pixel in chunk {
            if y_offset < y_end {
                img.put_pixel(x, y_offset, pixel);
                y_offset += 1;
            }
        }
    }
}
