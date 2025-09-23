// image_ops.rs
// Bildverarbeitungsfunktionen für Pixel-Sorting-Projekt

use image::{self, RgbaImage};
use crate::model::SortMode;
use crate::random_sort;
use crate::model::Model;

// Add rayon for parallel processing when available
use rayon::prelude::*;

pub fn brightness(px: &image::Rgba<u8>) -> u8 {
    let [r, g, b, _] = px.0;
    ((r as u16 + g as u16 + b as u16) / 3) as u8
}

pub fn sort_row_segment(img: &mut RgbaImage, y: u32, x_start: u32, x_end: u32, use_random: bool, sort_mode: SortMode, segment_buf: &mut Vec<image::Rgba<u8>>, random_prob: f32) {
    if use_random {
        let sort_func = match sort_mode {
            SortMode::Brightness => random_sort::brightness_f32,
            SortMode::Black => random_sort::red_value_f32,
            SortMode::White => random_sort::inverted_red_f32,
        };
        random_sort::apply_random_exclude_to_row(img, y, x_start, x_end, sort_func, random_prob);
    } else {
        segment_buf.clear();
        for x in x_start..x_end {
            segment_buf.push(*img.get_pixel(x, y));
        }
        segment_buf.sort_by_key(|px| match sort_mode {
            SortMode::Brightness => brightness(px),
            SortMode::Black => px[0] as u8,
            SortMode::White => 255 - px[0] as u8,
        });
        for (i, px) in segment_buf.iter().enumerate() {
            img.put_pixel(x_start + i as u32, y, *px);
        }
    }
}

pub fn sort_column_segment(img: &mut RgbaImage, x: u32, y_start: u32, y_end: u32, use_random: bool, sort_mode: SortMode, segment_buf: &mut Vec<image::Rgba<u8>>, random_prob: f32) {
    if use_random {
        let sort_func = match sort_mode {
            SortMode::Brightness => random_sort::brightness_f32,
            SortMode::Black => random_sort::red_value_f32,
            SortMode::White => random_sort::inverted_red_f32,
        };
        random_sort::apply_random_exclude_to_column(img, x, y_start, y_end, sort_func, random_prob);
    } else {
        segment_buf.clear();
        for y in y_start..y_end {
            segment_buf.push(*img.get_pixel(x, y));
        }
        segment_buf.sort_by_key(|px| match sort_mode {
            SortMode::Brightness => brightness(px),
            SortMode::Black => px[0] as u8,
            SortMode::White => 255 - px[0] as u8,
        });
        for (i, px) in segment_buf.iter().enumerate() {
            img.put_pixel(x, y_start + i as u32, *px);
        }
    }
}

pub fn get_next_segment_row_bright(img: &RgbaImage, mut x: u32, y: u32, width: u32, brightness_value: u8) -> (u32, u32) {
    // Advance to first pixel that meets or exceeds the brightness threshold
    while x < width && brightness(img.get_pixel(x, y)) < brightness_value { x += 1; }
    let start = x;
    // Continue while pixels meet or exceed the threshold (include equality)
    while x < width && brightness(img.get_pixel(x, y)) >= brightness_value { x += 1; }
    (start, x)
}

// Removed unnecessary `mut` from `y` parameter to avoid unused_mut warning. Use a local mutable variable inside.
pub fn get_next_segment_column_bright(img: &RgbaImage, x: u32, y: u32, height: u32, brightness_value: u8) -> (u32, u32) {
    // Advance to first pixel that meets or exceeds the brightness threshold
    let mut yy = y;
    while yy < height && brightness(img.get_pixel(x, yy)) < brightness_value { yy += 1; }
    let start = yy;
    // Continue while pixels meet or exceed the threshold (include equality)
    while yy < height && brightness(img.get_pixel(x, yy)) >= brightness_value { yy += 1; }
    (start, yy)
}

// Hauptfunktionen für horizontales und vertikales Sortieren
pub fn sort_and_update_pixels(model: &mut Model) {
    let mut img = model.img_original.clone();
    let (width, height) = (model.width, model.height);
    let use_random = model.random_exclude_mode;
    let sort_mode = model.sort_mode;
    let brightness_value = model.brightness_value;
    let random_prob = model.random_exclude_prob;
    
    // Reusable buffer for segments to avoid allocating per-segment
    let mut segment_buf: Vec<image::Rgba<u8>> = Vec::with_capacity(width as usize);
    
    // Spalten sortieren
    if !use_random {
        // Parallel compute new columns based on original img, then write back sequentially
        let src = img.clone();
        let columns: Vec<Vec<image::Rgba<u8>>> = (0..width).into_par_iter().map(|x| {
            let mut col: Vec<image::Rgba<u8>> = Vec::with_capacity(height as usize);
            for y in 0..height {
                col.push(*src.get_pixel(x, y));
            }
            let mut y = 0u32;
            while y < height {
                let (start, end) = get_next_segment_column_bright(&src, x, y, height, brightness_value);
                if start >= end || start >= height {
                    y = end + 1;
                    continue;
                }
                // collect, sort, and place
                let mut seg: Vec<image::Rgba<u8>> = (start..end).map(|yy| *src.get_pixel(x, yy)).collect();
                seg.sort_by_key(|px| match sort_mode {
                    SortMode::Brightness => brightness(px),
                    SortMode::Black => px[0] as u8,
                    SortMode::White => 255 - px[0] as u8,
                });
                for (i, px) in seg.into_iter().enumerate() {
                    col[(start as usize) + i] = px;
                }
                y = end + 1;
            }
            col
        }).collect();

        // write back
        for x in 0..width {
            let col = &columns[x as usize];
            for y in 0..height {
                img.put_pixel(x, y, col[y as usize]);
            }
        }
    } else {
        // Fallback to original sequential behavior when randomness is used
        for x in 0..width {
            let mut y = 0;
            while y < height {
                let (start, end) = get_next_segment_column_bright(&img, x, y, height, brightness_value);
                if start >= end || start >= height {
                    break;
                }
                sort_column_segment(&mut img, x, start, end, use_random, sort_mode, &mut segment_buf, random_prob);
                y = end + 1;
            }
        }
    }

    // Zeilen sortieren
    if !use_random {
        let src = img.clone();
        let rows: Vec<Vec<image::Rgba<u8>>> = (0..height).into_par_iter().map(|y| {
            let mut row: Vec<image::Rgba<u8>> = Vec::with_capacity(width as usize);
            for x in 0..width {
                row.push(*src.get_pixel(x, y));
            }
            let mut x = 0u32;
            while x < width {
                let (start, end) = get_next_segment_row_bright(&src, x, y, width, brightness_value);
                if start >= end || start >= width {
                    x = end + 1;
                    continue;
                }
                let mut seg: Vec<image::Rgba<u8>> = (start..end).map(|xx| *src.get_pixel(xx, y)).collect();
                seg.sort_by_key(|px| match sort_mode {
                    SortMode::Brightness => brightness(px),
                    SortMode::Black => px[0] as u8,
                    SortMode::White => 255 - px[0] as u8,
                });
                for (i, px) in seg.into_iter().enumerate() {
                    row[(start as usize) + i] = px;
                }
                x = end + 1;
            }
            row
        }).collect();

        // write back rows
        for y in 0..height {
            let row = &rows[y as usize];
            for x in 0..width {
                img.put_pixel(x, y, row[x as usize]);
            }
        }
    } else {
        for y in 0..height {
            let mut x = 0;
            while x < width {
                let (start, end) = get_next_segment_row_bright(&img, x, y, width, brightness_value);
                if start >= end || start >= width {
                    break;
                }
                sort_row_segment(&mut img, y, start, end, use_random, sort_mode, &mut segment_buf, random_prob);
                x = end + 1;
            }
        }
    }
    // Move the processed image into the model to avoid an extra full clone
    model.img_horizontal = img;
}

pub fn vertical_sort_and_update_pixels(model: &mut Model) {
    let mut img = model.img_horizontal.clone();
    let (width, height) = (model.width, model.height);
    let use_random = model.random_exclude_mode;
    let sort_mode = model.sort_mode;
    let brightness_value = model.brightness_value_vertical;
    let random_prob = model.random_exclude_prob;
    
    // Reusable buffer sized for vertical segments
    let mut segment_buf: Vec<image::Rgba<u8>> = Vec::with_capacity(height as usize);
    
    if !use_random {
        let src = img.clone();
        let columns: Vec<Vec<image::Rgba<u8>>> = (0..width).into_par_iter().map(|x| {
            let mut col: Vec<image::Rgba<u8>> = Vec::with_capacity(height as usize);
            for y in 0..height {
                col.push(*src.get_pixel(x, y));
            }
            let mut y = 0u32;
            while y < height {
                let (start, end) = get_next_segment_column_bright(&src, x, y, height, brightness_value);
                if start >= end || start >= height {
                    y = end + 1;
                    continue;
                }
                let mut seg: Vec<image::Rgba<u8>> = (start..end).map(|yy| *src.get_pixel(x, yy)).collect();
                seg.sort_by_key(|px| match sort_mode {
                    SortMode::Brightness => brightness(px),
                    SortMode::Black => px[0] as u8,
                    SortMode::White => 255 - px[0] as u8,
                });
                for (i, px) in seg.into_iter().enumerate() {
                    col[(start as usize) + i] = px;
                }
                y = end + 1;
            }
            col
        }).collect();

        for x in 0..width {
            let col = &columns[x as usize];
            for y in 0..height {
                img.put_pixel(x, y, col[y as usize]);
            }
        }
    } else {
        for x in 0..width {
            let mut y = 0;
            while y < height {
                let (start, end) = get_next_segment_column_bright(&img, x, y, height, brightness_value);
                if start >= end || start >= height {
                    break;
                }
                sort_column_segment(&mut img, x, start, end, use_random, sort_mode, &mut segment_buf, random_prob);
                y = end + 1;
            }
        }
    }
    // Move the processed image into the model to avoid an extra full clone
    model.img_vertical = img;
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbaImage, Rgba};

    #[test]
    fn test_get_next_segment_row_bright_inclusive() {
        // Create a 5x1 image with brightness: [10, 50, 70, 70, 20]
        let mut img = RgbaImage::new(5, 1);
        let vals = [10u8, 50u8, 70u8, 70u8, 20u8];
        for (x, v) in vals.iter().enumerate() {
            img.put_pixel(x as u32, 0, Rgba([*v, *v, *v, 255]));
        }
        // Threshold 50 should include pixels 1..4 (50,70,70)
        let (start, end) = get_next_segment_row_bright(&img, 0, 0, 5, 50);
        assert_eq!((start, end), (1, 4));

        // If starting at x=2 should find 2..4
        let (s2, e2) = get_next_segment_row_bright(&img, 2, 0, 5, 50);
        assert_eq!((s2, e2), (2, 4));

        // Threshold 70 should include pixels equal to 70 only (2..4 where 2 and 3 are 70)
        let (s3, e3) = get_next_segment_row_bright(&img, 0, 0, 5, 70);
        assert_eq!((s3, e3), (2, 4));

        // Threshold 80 should find no segment (start==end==5)
        let (s4, e4) = get_next_segment_row_bright(&img, 0, 0, 5, 80);
        assert_eq!((s4, e4), (5, 5));
    }

    #[test]
    fn test_get_next_segment_column_bright_inclusive() {
        // Create a 1x5 image with brightness down the column: [10, 50, 70, 70, 20]
        let mut img = RgbaImage::new(1, 5);
        let vals = [10u8, 50u8, 70u8, 70u8, 20u8];
        for (y, v) in vals.iter().enumerate() {
            img.put_pixel(0, y as u32, Rgba([*v, *v, *v, 255]));
        }
        // Threshold 50 should include y indices 1..4
        let (start, end) = get_next_segment_column_bright(&img, 0, 0, 5, 50);
        assert_eq!((start, end), (1, 4));

        // Threshold 70 should include indices 2..4
        let (s2, e2) = get_next_segment_column_bright(&img, 0, 0, 5, 70);
        assert_eq!((s2, e2), (2, 4));

        // Threshold 80 no segment
        let (s3, e3) = get_next_segment_column_bright(&img, 0, 0, 5, 80);
        assert_eq!((s3, e3), (5, 5));
    }
}
