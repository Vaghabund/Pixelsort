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

pub fn random_exclude(
    pixels: Vec<image::Rgba<u8>>,
    sort_func: fn(&image::Rgba<u8>) -> f32,
    lower: f32,
    upper: f32,
) -> Vec<Vec<image::Rgba<u8>>> {
    let mut chunks: Vec<Vec<image::Rgba<u8>>> = vec![];
    let mut group = vec![];
    
    for pixel in pixels {
        let random_val = rand::thread_rng().gen_range(0.0..1.0);
        
        if random_val >= lower && random_val <= upper {
            // Pixel zum Sortieren hinzufügen
            group.push(pixel);
        } else {
            // Aktuelle Gruppe sortieren und speichern, dann neue Gruppe starten
            if !group.is_empty() {
                group.sort_by_key(|px| (sort_func(px) * 1000.0) as u32);
                chunks.push(group.clone());
                group.clear();
            }
            // Einzelpixel als eigene "Gruppe" (bleibt unverändert)
            chunks.push(vec![pixel]);
        }
    }
    
    // Letzte Gruppe verarbeiten
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
) {
    let pixels: Vec<_> = (x_start..x_end)
        .map(|x| *img.get_pixel(x, y))
        .collect();
    
    // 30% Chance dass Pixel sortiert werden (0.3 bis 1.0)
    let chunks = random_exclude(pixels, sort_func, 0.3, 1.0);
    
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
) {
    let pixels: Vec<_> = (y_start..y_end)
        .map(|y| *img.get_pixel(x, y))
        .collect();
    
    // 30% Chance dass Pixel sortiert werden
    let chunks = random_exclude(pixels, sort_func, 0.3, 1.0);
    
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
